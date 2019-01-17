use gfx_hal::{buffer, device::Device, memory, pso, Backend, DescriptorPool, MemoryType};

pub struct PaletteUniform<B: Backend> {
    desc_pool: B::DescriptorPool,
    desc_set_layout: B::DescriptorSetLayout,
    desc_set: B::DescriptorSet,
    buffer: B::Buffer,
    memory: B::Memory,
}

impl<B: Backend> PaletteUniform<B> {
    pub unsafe fn new(
        device: &mut B::Device,
        memory_types: &[MemoryType],
        data: &[f32; 256],
    ) -> Self {
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

        let uniform_upload_size = data.len() as u64 * 4;
        println!("Uniform upload size {}", uniform_upload_size);
        let (memory, buffer) = {
            let mut buffer = device
                .create_buffer(uniform_upload_size, buffer::Usage::UNIFORM)
                .expect("Unable to create palette uniform buffer");

            let mem_req = device.get_buffer_requirements(&buffer);

            let memory_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type
                            .properties
                            .contains(memory::Properties::CPU_VISIBLE)
                })
                .expect("Palette uniform memory type not supported")
                .into();

            let memory = device.allocate_memory(memory_type, mem_req.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            let size = mem_req.size;

            {
                let mut data_target = device
                    .acquire_mapping_writer(&memory, 0..size)
                    .expect("Unable to acquire mapping writer");
                data_target[0..data.len()].copy_from_slice(data);
                device
                    .release_mapping_writer(data_target)
                    .expect("Unable to release mapping writer");
            }

            (memory, buffer)
        };

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
