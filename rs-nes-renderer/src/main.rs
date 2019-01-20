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

mod backend_resources;
mod nes_screen;
mod palette;
mod palette_uniform;
mod renderer_state;
mod swapchain_state;
mod vertex;

use rs_nes::{
    load_cart, Button, Cart, IInput, IPpu, Interrupt, Nes, NesRom, Nrom128, Nrom256, Uxrom,
};
use std::{
    env,
    fs::File,
    time::{Duration, Instant},
};
use winit::{ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    backend_resources::create_backend,
    renderer_state::{RenderStatus, RendererState},
    vertex::Vertex,
};

use rs_nes::{SCREEN_HEIGHT, SCREEN_WIDTH};

use gfx_hal::{format, image as i, window::Extent2D, Backend};

type FrameBufferFormat = format::Rgba8Srgb;
type ScreenBufferFormat = format::Rgba8Unorm;

trait SurfaceTrait {
    fn resize(&self, _size: winit::dpi::LogicalSize) {}
}

impl SurfaceTrait for <back::Backend as Backend>::Surface {
    #[cfg(feature = "gl")]
    fn resize(&self, size: winit::dpi::LogicalSize) {
        self.get_window()
            .resize(size.to_physical(self.get_window().get_hidpi_factor()));
    }
}

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

fn handle_input<C: Cart, B: SurfaceTrait>(
    surface: &B,
    events_loop: &mut EventsLoop,
    nes: &mut Nes<C>,
) -> InputStatus {
    let mut input_status = InputStatus::None;

    events_loop.poll_events(|event| {
        if let Event::WindowEvent { event, .. } = event {
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
                    surface.resize(dims);
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
    let mut events_loop = winit::EventsLoop::new();

    let window_builder = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(
            f64::from(DIMS.width),
            f64::from(DIMS.height),
        ))
        .with_title("rs-nes".to_string());

    let backend = create_backend(window_builder, &events_loop);

    let mut renderer_state = RendererState::new(backend);
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

            match handle_input(&renderer_state.surface, &mut events_loop, cpu) {
                InputStatus::Close => break 'running,
                InputStatus::RecreateSwapchain => recreate_swapchain = true,
                _ => (),
            }

            while cpu.step() != Interrupt::Nmi {}

            match renderer_state.render_frame(cpu.interconnect.ppu.screen(), recreate_swapchain) {
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
