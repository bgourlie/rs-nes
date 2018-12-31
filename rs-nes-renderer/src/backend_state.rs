use std::{cell::RefCell, fs, iter, mem::size_of, rc::Rc};

use hal::{
    buffer, command, format as f,
    format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
    image as i, memory as m,
    pass::{self, Subpass},
    pool,
    pso::{self, PipelineStage, ShaderStageFlags},
    queue::Submission,
    window::Extent2D,
    Adapter, Backbuffer, Backend, DescriptorPool, Device, FrameSync, Instance, Limits, MemoryType,
    PhysicalDevice, Primitive, QueueGroup, Surface, Swapchain, SwapchainConfig,
};

use crate::{adapter_state::AdapterState, window_state::WindowState};

pub struct BackendState<B: Backend> {
    pub surface: B::Surface,
    pub adapter: AdapterState<B>,
    #[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
    #[allow(dead_code)]
    window: winit::Window,
}

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
pub fn create_backend(
    window_state: &mut WindowState,
) -> (BackendState<back::Backend>, back::Instance) {
    let window = window_state
        .wb
        .take()
        .unwrap()
        .build(&window_state.events_loop)
        .unwrap();
    let instance = back::Instance::create("gfx-rs quad", 1);
    let surface = instance.create_surface(&window);
    let mut adapters = instance.enumerate_adapters();
    (
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface,
            window,
        },
        instance,
    )
}

#[cfg(feature = "gl")]
pub fn create_backend(window_state: &mut WindowState) -> (BackendState<back::Backend>, ()) {
    let window = {
        let builder =
            back::config_context(back::glutin::ContextBuilder::new(), ColorFormat::SELF, None)
                .with_vsync(true);
        back::glutin::GlWindow::new(
            window_state.wb.take().unwrap(),
            builder,
            &window_state.events_loop,
        )
        .unwrap()
    };

    let surface = back::Surface::from_window(window);
    let mut adapters = surface.enumerate_adapters();
    (
        BackendState {
            adapter: AdapterState::new(&mut adapters),
            surface,
        },
        (),
    )
}
