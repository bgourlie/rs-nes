use std::{cell::RefCell, rc::Rc};

use hal::{
    format as f,
    format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat},
    image as i,
    window::Extent2D,
    Backbuffer, Backend, Device, Surface, SwapchainConfig,
};

use crate::{backend_state::BackendState, device_state::DeviceState};

pub struct SwapchainState<B: Backend> {
    pub swapchain: Option<B::Swapchain>,
    pub backbuffer: Option<Backbuffer<B>>,
    device: Rc<RefCell<DeviceState<B>>>,
    pub extent: i::Extent,
    pub format: f::Format,
}

impl<B: Backend> SwapchainState<B> {
    pub unsafe fn new(
        backend: &mut BackendState<B>,
        device: Rc<RefCell<DeviceState<B>>>,
        dimensions: Extent2D,
    ) -> Self {
        let (caps, formats, _present_modes, _composite_alphas) = backend
            .surface
            .compatibility(&device.borrow().physical_device);
        println!("formats: {:?}", formats);
        let format = formats.map_or(ColorFormat::SELF, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .cloned()
                .unwrap_or(formats[0])
        });

        println!("Surface format: {:?}", format);
        let swap_config = SwapchainConfig::from_caps(&caps, format, dimensions);
        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = device
            .borrow()
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");

        SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            device,
            extent,
            format,
        }
    }
}

impl<B: Backend> Drop for SwapchainState<B> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .borrow()
                .device
                .destroy_swapchain(self.swapchain.take().unwrap());
        }
    }
}
