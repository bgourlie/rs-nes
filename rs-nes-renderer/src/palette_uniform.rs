use gfx_hal::{device::Device, pso, Backend, DescriptorPool};

pub struct PaletteUniform<B: Backend> {
    desc_pool: B::DescriptorPool,
    desc_set_layout: B::DescriptorSetLayout,
    desc_set: B::DescriptorSet,
    buffer: B::Buffer,
    memory: B::Memory,
}

impl<B: Backend> PaletteUniform<B> {
    pub unsafe fn new(device: &mut B::Device, memory: B::Memory, buffer: B::Buffer) -> Self {
        let mut desc_pool = device
            .create_descriptor_pool(
                1, // # of sets
                &[pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::UniformBuffer,
                    count: 1,
                }],
            )
            .expect("Unable to create image descriptor pool");

        let bindings = [pso::DescriptorSetLayoutBinding {
            binding: 0,
            ty: pso::DescriptorType::UniformBuffer,
            count: 1,
            stage_flags: pso::ShaderStageFlags::FRAGMENT,
            immutable_samplers: false,
        }];

        let desc_set_layout = device
            .create_descriptor_set_layout(&bindings, &[])
            .expect("Unable to create descriptor set layout");

        let desc_set = desc_pool
            .allocate_set(&desc_set_layout)
            .expect("Unable to allocate descriptor set");

        device.write_descriptor_sets(vec![pso::DescriptorSetWrite {
            binding: 0,
            array_offset: 0,
            descriptors: Some(pso::Descriptor::Buffer(&buffer, None..None)),
            set: &desc_set,
        }]);

        PaletteUniform {
            memory,
            buffer,
            desc_pool,
            desc_set_layout,
            desc_set,
        }
    }

    pub fn layout(&self) -> &B::DescriptorSetLayout {
        &self.desc_set_layout
    }

    pub fn descriptor_set(&self) -> &B::DescriptorSet {
        &self.desc_set
    }

    pub fn destroy(self, device: &B::Device) {
        unsafe {
            device.destroy_buffer(self.buffer);
            device.free_memory(self.memory);
            device.destroy_descriptor_set_layout(self.desc_set_layout);
            device.destroy_descriptor_pool(self.desc_pool);
        }
    }
}
