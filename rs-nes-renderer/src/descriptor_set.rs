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

use crate::device_state::DeviceState;

pub struct DescSetLayout<B: Backend> {
    pub layout: Option<B::DescriptorSetLayout>,
    pub device: Rc<RefCell<DeviceState<B>>>,
}

impl<B: Backend> DescSetLayout<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        bindings: Vec<pso::DescriptorSetLayoutBinding>,
    ) -> Self {
        let desc_set_layout = device
            .borrow()
            .device
            .create_descriptor_set_layout(bindings, &[])
            .ok();

        DescSetLayout {
            layout: desc_set_layout,
            device,
        }
    }

    pub unsafe fn create_desc_set(self, desc_pool: &mut B::DescriptorPool) -> DescSet<B> {
        let desc_set = desc_pool
            .allocate_set(self.layout.as_ref().unwrap())
            .unwrap();
        DescSet {
            layout: self,
            set: Some(desc_set),
        }
    }
}

impl<B: Backend> Drop for DescSetLayout<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_descriptor_set_layout(self.layout.take().unwrap());
        }
    }
}

pub struct DescSet<B: Backend> {
    pub set: Option<B::DescriptorSet>,
    pub layout: DescSetLayout<B>,
}

pub struct DescSetWrite<W> {
    pub binding: pso::DescriptorBinding,
    pub array_offset: pso::DescriptorArrayIndex,
    pub descriptors: W,
}

impl<B: Backend> DescSet<B> {
    pub unsafe fn write_to_state<'a, 'b: 'a, W>(
        &'b mut self,
        write: Vec<DescSetWrite<W>>,
        device: &mut B::Device,
    ) where
        W: IntoIterator,
        W::Item: std::borrow::Borrow<pso::Descriptor<'a, B>>,
    {
        let set = self.set.as_ref().unwrap();
        let write: Vec<_> = write
            .into_iter()
            .map(|d| pso::DescriptorSetWrite {
                binding: d.binding,
                array_offset: d.array_offset,
                descriptors: d.descriptors,
                set,
            })
            .collect();
        device.write_descriptor_sets(write);
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.layout.layout.as_ref().unwrap()
    }
}
