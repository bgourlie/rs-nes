use std::{cell::RefCell, fs, io::Read, iter, mem::size_of, rc::Rc};

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

pub struct DeviceState<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B, ::hal::Graphics>,
}

impl<B: Backend> DeviceState<B> {
    pub fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self {
        let (device, queues) = adapter
            .open_with::<_, ::hal::Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        DeviceState {
            device,
            queues,
            physical_device: adapter.physical_device,
        }
    }
}
