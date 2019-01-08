use gfx_hal::{pso, Backend, DescriptorPool, Device};

pub struct DescSetLayout<B: Backend> {
    pub layout: Option<B::DescriptorSetLayout>,
}

impl<B: Backend> DescSetLayout<B> {
    pub fn new(device: &B::Device, bindings: Vec<pso::DescriptorSetLayoutBinding>) -> Self {
        let desc_set_layout = unsafe { device.create_descriptor_set_layout(bindings, &[]).ok() };

        DescSetLayout {
            layout: desc_set_layout,
        }
    }

    pub unsafe fn create_desc_set(mut self, desc_pool: &mut B::DescriptorPool) -> DescSet<B> {
        let layout = self.layout.take().expect("Layout shouldn't be None");
        let desc_set = desc_pool.allocate_set(&layout).unwrap();

        DescSet {
            layout: Some(layout),
            set: Some(desc_set),
        }
    }
}

pub struct DescSet<B: Backend> {
    // TODO: This appears to be a transient resource that we don't need to hold onto or destroy
    pub set: Option<B::DescriptorSet>,
    pub layout: Option<B::DescriptorSetLayout>,
}

impl<B: Backend> DescSet<B> {
    pub fn destroy_resources(state: &mut Self, device: &B::Device) {
        unsafe {
            device.destroy_descriptor_set_layout(
                state
                    .layout
                    .take()
                    .expect("Descriptor set layout shouldn't be None"),
            );
        }
    }
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

    pub fn layout(&self) -> &B::DescriptorSetLayout {
        self.layout.as_ref().unwrap()
    }
}

pub struct DescSetWrite<W> {
    pub binding: pso::DescriptorBinding,
    pub array_offset: pso::DescriptorArrayIndex,
    pub descriptors: W,
}
