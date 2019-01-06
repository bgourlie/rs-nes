use gfx_hal::{Backend, Instance};

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
    use gfx_hal::format::AsFormat;
    let window = {
        let builder = back::config_context(
            back::glutin::ContextBuilder::new(),
            crate::FrameBufferFormat::SELF,
            None,
        )
        .with_vsync(true);
        back::glutin::GlWindow::new(
            window_state.wb.take().unwrap(),
            builder,
            &window_state.events_loop,
        )
        .expect("Unable to create window")
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

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx12",
    feature = "metal",
    feature = "gl"
)))]
pub fn create_backend(_window_state: &mut WindowState) -> (BackendState<back::Backend>, ()) {
    unimplemented!()
}
