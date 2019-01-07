use gfx_hal::{
    pool::{CommandPool, CommandPoolCreateFlags},
    Adapter, Backend, Device, Graphics, QueueGroup, Surface,
};

pub struct DeviceState<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B, Graphics>,
}

impl<B: Backend> DeviceState<B> {
    pub fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self {
        let (device, queues) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        DeviceState {
            device,
            queues,
            physical_device: adapter.physical_device,
        }
    }

    pub fn create_command_pool(&self) -> CommandPool<B, Graphics> {
        unsafe {
            self.device
                .create_command_pool_typed(&self.queues, CommandPoolCreateFlags::empty())
                .expect("Can't create command pool")
        }
    }
}
