use gfx_hal::{
    format as f,
    format::{AsFormat, ChannelType},
    image as i,
    window::Extent2D,
    Backbuffer, Backend, Device, Surface, SwapchainConfig,
};

use crate::{backend_state::BackendState, device_state::DeviceState, FrameBufferFormat};

pub struct SwapchainState<B: Backend> {
    pub swapchain: Option<B::Swapchain>,
    pub backbuffer: Option<Backbuffer<B>>,
    pub extent: i::Extent,
    pub format: f::Format,
}

impl<B: Backend> SwapchainState<B> {
    pub unsafe fn new(
        backend: &mut BackendState<B>,
        device: &DeviceState<B>,
        dimensions: Extent2D,
    ) -> Self {
        let (caps, formats, _present_modes, _composite_alphas) =
            backend.surface.compatibility(&device.physical_device);
        println!("formats: {:?}", formats);
        let format = formats.map_or(FrameBufferFormat::SELF, |formats| {
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
            .device
            .create_swapchain(&mut backend.surface, swap_config, None)
            .expect("Can't create swapchain");

        SwapchainState {
            swapchain: Some(swapchain),
            backbuffer: Some(backbuffer),
            extent,
            format,
        }
    }

    pub fn destroy_resources(state: &mut Self, device: &B::Device) {
        unsafe {
            device.destroy_swapchain(
                state
                    .swapchain
                    .take()
                    .expect("Swapchain state shouldn't be None"),
            );
        }
    }
}
