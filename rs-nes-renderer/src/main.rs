#![cfg_attr(
    not(any(
        feature = "vulkan",
        feature = "dx12",
        feature = "metal",
        feature = "gl"
    )),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(not(any(
    feature = "vulkan",
    feature = "dx12",
    feature = "metal",
    feature = "gl"
)))]
extern crate gfx_backend_empty as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

mod adapter_state;
mod backend_state;
mod buffer_state;
mod descriptor_set;
mod device_state;
mod framebuffer_state;
mod image_state;
mod pipeline_state;
mod render_pass_state;
mod renderer_state;
mod swapchain_state;
mod uniform;
mod vertex;
mod window_state;

use log::info;
use rs_nes::{load_cart, Cart, Nes, NesRom, Nrom128, Nrom256, Uxrom};
use std::{
    env,
    fs::File,
    time::{Duration, Instant},
};

use crate::{
    backend_state::create_backend,
    renderer_state::{InputStatus, RenderStatus, RendererState},
    vertex::Vertex,
    window_state::WindowState,
};

use gfx_hal::{format as f, image as i, window::Extent2D, Backend, Device};

pub const BYTES_PER_PIXEL: usize = 4;
pub const IMAGE_WIDTH: usize = 256;
pub const IMAGE_HEIGHT: usize = 240;

const DIMS: Extent2D = Extent2D {
    width: 1024,
    height: 768,
};

pub const QUAD: [Vertex; 6] = [
    Vertex {
        a_pos: [-0.5, 0.33],
        a_uv: [0.0, 1.0],
    },
    Vertex {
        a_pos: [0.5, 0.33],
        a_uv: [1.0, 1.0],
    },
    Vertex {
        a_pos: [0.5, -0.33],
        a_uv: [1.0, 0.0],
    },
    Vertex {
        a_pos: [-0.5, 0.33],
        a_uv: [0.0, 1.0],
    },
    Vertex {
        a_pos: [0.5, -0.33],
        a_uv: [1.0, 0.0],
    },
    Vertex {
        a_pos: [-0.5, -0.33],
        a_uv: [0.0, 0.0],
    },
];

pub const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

trait SurfaceTrait {
    #[cfg(feature = "gl")]
    fn get_window_t(&self) -> &back::glutin::GlWindow;
}

impl SurfaceTrait for <back::Backend as Backend>::Surface {
    #[cfg(feature = "gl")]
    fn get_window_t(&self) -> &back::glutin::GlWindow {
        self.get_window()
    }
}

#[cfg(any(
    feature = "vulkan",
    feature = "dx12",
    feature = "metal",
    feature = "gl"
))]
fn main() {
    env_logger::init();

    let rom_path = env::args().last().expect("Unable to determine rom path");
    let mut rom_file = File::open(rom_path).expect("Unable to open ROM file");
    let rom = NesRom::load(&mut rom_file).expect("Unable to load ROM");
    info!("ROM INFORMATION");
    info!("{:?}", rom);
    match rom.mapper {
        0 => match rom.prg_rom_banks {
            1 => {
                let cart = Nrom128::new(&rom).expect("Unable to map ROM to cart");
                let mut cpu = load_cart(cart).expect("Unable to load cart");
                run(&mut cpu);
            }
            2 => {
                let cart = Nrom256::new(&rom).expect("Unable to map ROM to cart");
                let mut cpu = load_cart(cart).expect("Unable to load cart");
                run(&mut cpu);
            }
            _ => panic!("Unsupported NROM cart"),
        },
        2 => {
            let cart = Uxrom::new(&rom).expect("Unable to map ROM to cart");
            let mut cpu = load_cart(cart).expect("Unable to load cart");
            run(&mut cpu);
        }
        _ => panic!("Mapper {} not supported", rom.mapper),
    }
}

fn run<C: Cart>(_cpu: &mut Nes<C>) {
    let mut screen_buffer = vec![255_u8; IMAGE_WIDTH * IMAGE_HEIGHT * BYTES_PER_PIXEL];

    let mut window = WindowState::new();
    let (backend, _instance) = create_backend(&mut window);

    let mut renderer_state = unsafe { RendererState::new(backend, window) };
    let start_time = Instant::now();
    let mut recreate_swapchain = false;
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();
    let fixed_time_stamp = Duration::new(0, 16_666_667);
    'running: loop {
        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            match renderer_state.handle_input() {
                InputStatus::Close => break 'running,
                InputStatus::RecreateSwapchain => recreate_swapchain = true,
                _ => (),
            }

            let t_mod = ((now - start_time).as_secs() % 255) as usize + 1;
            for y in 0..IMAGE_HEIGHT {
                for x in 0..IMAGE_WIDTH {
                    let i = ((y * IMAGE_WIDTH + x) * BYTES_PER_PIXEL) as usize;
                    screen_buffer[i] = (y % t_mod) as u8;
                    screen_buffer[i + 1] = x as u8;
                    screen_buffer[i + 2] = ((x + y) % 255) as u8;
                }
            }

            renderer_state
                .image
                .buffer
                .as_mut()
                .unwrap()
                .update_data(&screen_buffer);

            let staging_pool = unsafe {
                let mut staging_pool = renderer_state.device.borrow().create_command_pool();
                renderer_state.image.copy_buffer_to_texture(
                    &mut renderer_state.device.borrow_mut(),
                    &mut staging_pool,
                );
                staging_pool
            };

            renderer_state.image.wait_for_transfer_completion();

            unsafe {
                renderer_state
                    .device
                    .borrow()
                    .device
                    .destroy_command_pool(staging_pool.into_raw());
            }

            match renderer_state.render_frame(recreate_swapchain) {
                RenderStatus::RecreateSwapchain => recreate_swapchain = true,
                RenderStatus::NormalAndSwapchainRecreated => recreate_swapchain = false,
                _ => (),
            }
        }
        std::thread::sleep(fixed_time_stamp - accumulator);
    }
}

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx12",
    feature = "metal",
    feature = "gl"
)))]
fn main() {
    println!("You need to enable the native API feature (vulkan/metal) in order to test the LL");
}
