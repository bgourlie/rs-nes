use gfx_hal::{buffer, device::Device, memory, pso, Backend, MemoryType};

use crate::descriptor_set::{DescSet, DescSetWrite};

pub struct PaletteUniform<B: Backend> {
    buffer: Option<B::Buffer>,
    memory: Option<B::Memory>,
    desc: DescSet<B>,
}

impl<B: Backend> PaletteUniform<B> {
    pub unsafe fn new(
        device: &mut B::Device,
        memory_types: &[MemoryType],
        data: &[f32; 256],
        mut desc: DescSet<B>,
    ) -> Self {
        let uniform_upload_size = data.len() as u64 * 4;
        println!("Uniform upload size {}", uniform_upload_size);
        let (uniform_memory, uniform_buffer) = {
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

        desc.write_to_state(
            vec![DescSetWrite {
                binding: 0,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Buffer(&uniform_buffer, None..None)),
            }],
            device,
        );

        PaletteUniform {
            memory: Some(uniform_memory),
            buffer: Some(uniform_buffer),
            desc,
        }
    }

    pub fn layout(&self) -> &B::DescriptorSetLayout {
        &self.desc.layout()
    }

    pub fn descriptor_set(&self) -> &B::DescriptorSet {
        self.desc.set.as_ref().unwrap()
    }

    pub fn destroy_resources(state: &mut Self, device: &B::Device) {
        unsafe {
            device.destroy_buffer(state.buffer.take().expect("Buffer shouldn't be None"));
            device.free_memory(state.memory.take().expect("Memory shouldn't be None"));
            DescSet::destroy_resources(&mut state.desc, device);
        }
    }
}
