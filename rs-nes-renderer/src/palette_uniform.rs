use std::{cell::RefCell, rc::Rc};

use gfx_hal::{buffer, device::Device, memory, pso, Backend, MemoryType};

use crate::{
    descriptor_set::{DescSet, DescSetWrite},
    device_state::DeviceState,
};

pub struct PaletteUniform<B: Backend> {
    memory: Option<B::Memory>,
    buffer: Option<B::Buffer>,
    device: Rc<RefCell<DeviceState<B>>>,
    desc: Option<DescSet<B>>,
}

impl<B: Backend> PaletteUniform<B> {
    pub unsafe fn new(
        device: Rc<RefCell<DeviceState<B>>>,
        memory_types: &[MemoryType],
        data: &[f32; 256],
        mut desc: DescSet<B>,
        binding: u32,
    ) -> Self {
        let uniform_upload_size = data.len() as u64 * 4;
        println!("Uniform upload size {}", uniform_upload_size);
        let (uniform_memory, uniform_buffer) = {
            let device = &device.borrow().device;

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
                binding,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Buffer(&uniform_buffer, None..None)),
            }],
            &mut device.borrow_mut().device,
        );

        PaletteUniform {
            device,
            memory: Some(uniform_memory),
            buffer: Some(uniform_buffer),
            desc: Some(desc),
        }
    }

    pub fn layout(&self) -> &B::DescriptorSetLayout {
        self.desc.as_ref().unwrap().get_layout()
    }

    pub fn descriptor_set(&self) -> &B::DescriptorSet {
        self.desc.as_ref().unwrap().set.as_ref().unwrap()
    }
}

impl<B: Backend> Drop for PaletteUniform<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_buffer(self.buffer.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }
    }
}
