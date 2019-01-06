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
mod descriptor_set;
mod device_state;
mod framebuffer_state;
mod nes_screen_buffer;
mod pipeline_state;
mod render_pass_state;
mod renderer_state;
mod swapchain_state;
mod uniform;
mod vertex;
mod window_state;

use rs_nes::{
    load_cart, Button, Cart, IInput, IPpu, Interrupt, Nes, NesRom, Nrom128, Nrom256, Uxrom,
};
use std::{
    env,
    fs::File,
    time::{Duration, Instant},
};
use winit::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    backend_state::{create_backend, BackendState},
    renderer_state::{RenderStatus, RendererState},
    vertex::Vertex,
    window_state::WindowState,
};

use rs_nes::{SCREEN_HEIGHT, SCREEN_WIDTH};

use gfx_hal::{format, image as i, window::Extent2D, Backend, Device};

type FrameBufferFormat = format::Rgba8Srgb;
type ScreenBufferFormat = format::Rgba8Unorm;

const DIMS: Extent2D = Extent2D {
    width: SCREEN_WIDTH as u32,
    height: SCREEN_HEIGHT as u32,
};

pub const QUAD: [Vertex; 6] = [
    Vertex {
        a_pos: [-1.0, 1.0],
        a_uv: [0.0, 1.0],
    },
    Vertex {
        a_pos: [1.0, 1.0],
        a_uv: [1.0, 1.0],
    },
    Vertex {
        a_pos: [1.0, -1.0],
        a_uv: [1.0, 0.0],
    },
    Vertex {
        a_pos: [-1.0, 1.0],
        a_uv: [0.0, 1.0],
    },
    Vertex {
        a_pos: [1.0, -1.0],
        a_uv: [1.0, 0.0],
    },
    Vertex {
        a_pos: [-1.0, -1.0],
        a_uv: [0.0, 0.0],
    },
];

pub const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: format::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

pub enum InputStatus {
    None,
    Close,
    RecreateSwapchain,
}

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
    let rom_path = env::args().last().expect("Unable to determine rom path");
    let mut rom_file = File::open(rom_path).expect("Unable to open ROM file");
    let rom = NesRom::load(&mut rom_file).expect("Unable to load ROM");
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

fn handle_input<C: Cart>(
    _backend: &BackendState<back::Backend>,
    window: &mut WindowState,
    nes: &mut Nes<C>,
) -> InputStatus {
    let mut input_status = InputStatus::None;

    window.events_loop.poll_events(|event| {
        if let Event::WindowEvent { event, .. } = event {
            #[allow(unused_variables)]
            match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => input_status = InputStatus::Close,
                WindowEvent::Resized(dims) => {
                    #[cfg(feature = "gl")]
                    _backend.surface.get_window_t().resize(
                        dims.to_physical(_backend.surface.get_window_t().get_hidpi_factor()),
                    );
                    input_status = InputStatus::RecreateSwapchain;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        },
                    ..
                } => {
                    if let Some(kc) = virtual_keycode {
                        let button = match kc {
                            VirtualKeyCode::W => Some(Button::Up),
                            VirtualKeyCode::A => Some(Button::Left),
                            VirtualKeyCode::S => Some(Button::Down),
                            VirtualKeyCode::D => Some(Button::Right),
                            VirtualKeyCode::J => Some(Button::B),
                            VirtualKeyCode::K => Some(Button::A),
                            VirtualKeyCode::Return => Some(Button::Start),
                            VirtualKeyCode::LShift | VirtualKeyCode::RShift => Some(Button::Select),
                            _ => None,
                        };

                        if let Some(button) = button {
                            match state {
                                ElementState::Pressed => {
                                    nes.interconnect.input.player1_press(button)
                                }
                                ElementState::Released => {
                                    nes.interconnect.input.player1_release(button)
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    });

    input_status
}

fn run<C: Cart>(cpu: &mut Nes<C>) {
    let mut window = WindowState::new();
    let (backend, _instance) = create_backend(&mut window);

    let mut renderer_state = unsafe { RendererState::new(backend) };
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

            match handle_input(&renderer_state.backend, &mut window, cpu) {
                InputStatus::Close => break 'running,
                InputStatus::RecreateSwapchain => recreate_swapchain = true,
                _ => (),
            }

            loop {
                if cpu.step() == Interrupt::Nmi {
                    renderer_state
                        .nes_screen_buffer
                        .update_buffer_data(cpu.interconnect.ppu.screen());
                    break;
                }
            }

            let staging_pool = unsafe {
                let mut staging_pool = renderer_state.device.borrow().create_command_pool();
                // FIXME: Following line causing huge memory leak with dx12 backend
                renderer_state.nes_screen_buffer.copy_buffer_to_texture(
                    &mut renderer_state.device.borrow_mut(),
                    &mut staging_pool,
                );
                staging_pool
            };

            renderer_state
                .nes_screen_buffer
                .wait_for_transfer_completion();

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
