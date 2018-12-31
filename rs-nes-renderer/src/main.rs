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

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gfx_hal as hal;
extern crate glsl_to_spirv;
extern crate winit;

mod adapter_state;
mod backend_state;
mod buffer_state;
mod color;
mod descriptor_set;
mod device_state;
mod dimensions;
mod framebuffer_state;
mod image_state;
mod pipeline_state;
mod render_pass_state;
mod renderer_state;
mod swapchain_state;
mod uniform;
mod vertex;
mod window_state;

use crate::{
    backend_state::create_backend, renderer_state::RendererState, vertex::Vertex,
    window_state::WindowState,
};

use hal::{format as f, image as i, window::Extent2D};

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

pub trait SurfaceTrait {
    #[cfg(feature = "gl")]
    fn get_window_t(&self) -> &back::glutin::GlWindow;
}

impl SurfaceTrait for <back::Backend as hal::Backend>::Surface {
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

    let mut window = WindowState::new();
    let (backend, _instance) = create_backend(&mut window);

    let mut renderer_state = unsafe { RendererState::new(backend, window) };
    renderer_state.mainloop();
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
