use crate::DIMS;

pub struct WindowState {
    pub events_loop: winit::EventsLoop,
    pub wb: Option<winit::WindowBuilder>,
}

impl WindowState {
    pub fn new() -> WindowState {
        let events_loop = winit::EventsLoop::new();

        let wb = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(
                f64::from(DIMS.width),
                f64::from(DIMS.height),
            ))
            .with_title("quad".to_string());

        WindowState {
            events_loop,
            wb: Some(wb),
        }
    }
}
