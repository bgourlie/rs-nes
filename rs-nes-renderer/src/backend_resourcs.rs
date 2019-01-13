use gfx_hal::{Adapter, MemoryType, Limits, Backend, Instance, PhysicalDevice};
use winit::{EventsLoop, Window, WindowBuilder};

pub struct BackendResources<B: Backend> {
    surface: B::Surface,
    window: Option<Window>,
    adapter: Adapter<B>,
    memory_types: Vec<MemoryType>,
    limits: Limits,
}

impl<B: Backend> BackendResources<B> {
    pub fn take(self) -> (B::Surface, Adapter<B>, Limits, Vec<MemoryType>, Option<Window>) {
        (
            self.surface,
            self.adapter,
            self.limits,
            self.memory_types,
            self.window
        )
    }
}

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
pub fn create_backend(
    window: WindowBuilder,
    events_loop: &EventsLoop,
) -> BackendResources<back::Backend> {
    let instance = back::Instance::create("RS-NES", 1);
    let window = window.build(&events_loop).expect("Unable to build window");
    let surface = instance.create_surface(&window);
    let adapter = instance.enumerate_adapters().remove(0);
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let limits = adapter.physical_device.limits();

    BackendResources {
        adapter,
        surface,
        memory_types,
        limits,
        window: Some(window),
    }
}

#[cfg(feature = "gl")]
pub fn create_backend(
    window_builder: WindowBuilder,
    events_loop: &EventsLoop,
) -> BackendResources<back::Backend> {
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
    let adapter = surface.enumerate_adapters().remove(0);
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let limits = adapter.physical_device.limits();
    BackendResources {
        adapter,
        surface,
        memory_types,
        limits,
        window: None,
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
) -> BackendResources<back::Backend> {
    unimplemented!()
}
