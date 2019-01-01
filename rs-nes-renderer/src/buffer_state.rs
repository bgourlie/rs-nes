use std::{cell::RefCell, mem::size_of, rc::Rc};

use gfx_hal::{buffer, memory as m, Backend, Device, MemoryType};

use crate::{adapter_state::AdapterState, device_state::DeviceState};

pub struct BufferState<B: Backend> {
    memory: Option<B::Memory>,
    buffer: Option<B::Buffer>,
    device: Rc<RefCell<DeviceState<B>>>,
    size: u64,
}

impl<B: Backend> BufferState<B> {
    pub fn get_buffer(&self) -> &B::Buffer {
        self.buffer.as_ref().unwrap()
    }

    pub unsafe fn new<T>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        data_source: &[T],
        usage: buffer::Usage,
        memory_types: &[MemoryType],
    ) -> Self
    where
        T: Copy,
    {
        let memory: B::Memory;
        let mut buffer: B::Buffer;
        let size: u64;

        let stride = size_of::<T>() as u64;
        let upload_size = data_source.len() as u64 * stride;

        {
            let device = &device_ptr.borrow().device;

            buffer = device.create_buffer(upload_size, usage).unwrap();
            let mem_req = device.get_buffer_requirements(&buffer);

            // A note about performance: Using CPU_VISIBLE memory is convenient because it can be
            // directly memory mapped and easily updated by the CPU, but it is very slow and so should
            // only be used for small pieces of data that need to be updated very frequently. For something like
            // a vertex buffer that may be much larger and should not change frequently, you should instead
            // use a DEVICE_LOCAL buffer that gets filled by copying data from a CPU_VISIBLE staging buffer.
            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(m::Properties::CPU_VISIBLE)
                })
                .unwrap()
                .into();

            memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            size = mem_req.size;

            // TODO: check transitions: read/write mapping and vertex buffer read
            {
                let mut data_target = device
                    .acquire_mapping_writer::<T>(&memory, 0..size)
                    .unwrap();
                data_target[0..data_source.len()].copy_from_slice(data_source);
                device.release_mapping_writer(data_target).unwrap();
            }
        }

        BufferState {
            memory: Some(memory),
            buffer: Some(buffer),
            device: device_ptr,
            size,
        }
    }

    pub fn update_buffer_data<T>(&mut self, data_source: &[T])
    where
        T: Copy,
    {
        let device = &self.device.borrow().device;
        let stride = size_of::<T>() as u64;
        let upload_size = data_source.len() as u64 * stride;
        assert!(upload_size <= self.size);
        unsafe {
            let mut data_target = device
                .acquire_mapping_writer::<T>(self.memory.as_ref().unwrap(), 0..self.size)
                .unwrap();
            data_target[0..data_source.len()].copy_from_slice(data_source);
            device.release_mapping_writer(data_target).unwrap();
        }
    }

    pub unsafe fn new_texture(
        width: u32,
        height: u32,
        stride: u32,
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        device: &B::Device,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
    ) -> (Self, u32) {
        let row_alignment_mask = adapter.limits.min_buffer_copy_pitch_alignment as u32 - 1;

        let row_pitch = (width * stride + row_alignment_mask) & !row_alignment_mask;
        let upload_size = u64::from(height * row_pitch);

        let mut buffer = device.create_buffer(upload_size, usage).unwrap();
        let mem_reqs = device.get_buffer_requirements(&buffer);

        let upload_type = adapter
            .memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                mem_reqs.type_mask & (1 << id) != 0
                    && mem_type.properties.contains(m::Properties::CPU_VISIBLE)
            })
            .unwrap()
            .into();

        let memory = device.allocate_memory(upload_type, mem_reqs.size).unwrap();
        device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();

        (
            BufferState {
                memory: Some(memory),
                buffer: Some(buffer),
                device: device_ptr,
                size: mem_reqs.size,
            },
            row_pitch,
        )
    }
}

impl<B: Backend> Drop for BufferState<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;
        unsafe {
            device.destroy_buffer(self.buffer.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }
    }
}
