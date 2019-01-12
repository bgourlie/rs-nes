use crate::adapter_state::AdapterState;
use gfx_hal::{Backend, Instance};
use winit::{EventsLoop, Window, WindowBuilder};

pub struct BackendState<B: Backend> {
    pub surface: B::Surface,
    pub adapter: AdapterState<B>,
    _window: Option<Window>, // Required for non-GL backends to prevent it from dropping
}

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
pub fn create_backend(
    window: WindowBuilder,
    events_loop: &EventsLoop,
) -> BackendState<back::Backend> {
    let instance = back::Instance::create("RS-NES", 1);
    let window = window.build(&events_loop).expect("Unable to build window");
    let surface = instance.create_surface(&window);
    let mut adapters = instance.enumerate_adapters();
    BackendState {
        adapter: AdapterState::new(&mut adapters),
        surface,
        _window: Some(window),
    }
}

#[cfg(feature = "gl")]
pub fn create_backend(
    window_builder: WindowBuilder,
    events_loop: &EventsLoop,
) -> BackendState<back::Backend> {
    use gfx_hal::format::AsFormat;
    let window = {
        let builder = back::config_context(
            back::glutin::ContextBuilder::new(),
            crate::FrameBufferFormat::SELF,
            None,
        )
        .with_vsync(true);
        back::glutin::GlWindow::new(window_builder, builder, &events_loop)
            .expect("Unable to create window")
    };

    let surface = back::Surface::from_window(window);
    let mut adapters = surface.enumerate_adapters();
    BackendState {
        adapter: AdapterState::new(&mut adapters),
        surface,
        _window: None,
    }
}

#[cfg(not(any(
    feature = "vulkan",
    feature = "dx12",
    feature = "metal",
    feature = "gl"
)))]
pub fn create_backend(
    _window_builder: WindowBuilder,
    events_loop: &EventsLoop,
) -> BackendState<back::Backend> {
    unimplemented!()
}
