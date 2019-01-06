use std::{cell::RefCell, iter, rc::Rc};

use gfx_hal::{
    buffer, command, format as f,
    format::{AsFormat, Swizzle},
    image as i, memory as m, memory,
    pso::{self, PipelineStage},
    Backend, CommandPool, Device, Graphics, Supports, Transfer,
};

use crate::{
    adapter_state::AdapterState,
    descriptor_set::{DescSet, DescSetWrite},
    device_state::DeviceState,
    ScreenBufferFormat, COLOR_RANGE,
};

pub struct NesScreenBuffer<B: Backend> {
    device: Rc<RefCell<DeviceState<B>>>,
    desc: DescSet<B>,
    staging_buffer: Option<B::Buffer>,
    staging_buffer_memory: Option<B::Memory>,
    sampler: Option<B::Sampler>,
    image_view: Option<B::ImageView>,
    image: Option<B::Image>,
    texture_memory: Option<B::Memory>,
    image_transfer_fence: Option<B::Fence>,
    dimensions: (u32, u32),
    row_pitch: u32,
    row_alignment_mask: u32,
    stride: u32,
    staging_buffer_size: u64,
}

impl<B: Backend> NesScreenBuffer<B> {
    pub unsafe fn new<T: Supports<Transfer>>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        image_width: u32,
        image_height: u32,
        mut desc: DescSet<B>,
        adapter: &AdapterState<B>,
    ) -> Self {
        let stride = 4;
        let row_alignment_mask = adapter.limits.min_buffer_copy_pitch_alignment as u32 - 1;
        let row_pitch = (image_width * stride + row_alignment_mask) & !row_alignment_mask;
        let upload_size = u64::from(image_height * row_pitch);
        println!("Row alignment mask: {}", row_alignment_mask);

        let (
            desc,
            staging_buffer,
            staging_buffer_memory,
            staging_buffer_memory_requirements,
            sampler,
            image_view,
            image,
            texture_memory,
            image_transfer_fence,
        ) = {
            let device = &mut device_ptr.borrow_mut().device;

            // staging buffer
            let mut staging_buffer = device
                .create_buffer(upload_size, buffer::Usage::TRANSFER_SRC)
                .unwrap();
            let staging_buffer_memory_requirements =
                device.get_buffer_requirements(&staging_buffer);

            let staging_buffer_memory_type = adapter
                .memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    staging_buffer_memory_requirements.type_mask & (1 << id) != 0
                        && mem_type
                            .properties
                            .contains(memory::Properties::CPU_VISIBLE)
                })
                .unwrap()
                .into();

            let staging_buffer_memory = device
                .allocate_memory(
                    staging_buffer_memory_type,
                    staging_buffer_memory_requirements.size,
                )
                .unwrap();
            device
                .bind_buffer_memory(&staging_buffer_memory, 0, &mut staging_buffer)
                .unwrap();
            // end staging buffer

            let kind = i::Kind::D2(image_width as i::Size, image_height as i::Size, 1, 1);
            let mut image = device
                .create_image(
                    kind,
                    1,
                    ScreenBufferFormat::SELF,
                    i::Tiling::Optimal,
                    i::Usage::TRANSFER_DST | i::Usage::SAMPLED,
                    i::ViewCapabilities::empty(),
                )
                .unwrap(); // TODO: usage
            let req = device.get_image_requirements(&image);

            let device_type = adapter
                .memory_types
                .iter()
                .enumerate()
                .position(|(id, memory_type)| {
                    req.type_mask & (1 << id) != 0
                        && memory_type.properties.contains(m::Properties::DEVICE_LOCAL)
                })
                .unwrap()
                .into();

            let texture_memory = device.allocate_memory(device_type, req.size).unwrap();

            device
                .bind_image_memory(&texture_memory, 0, &mut image)
                .unwrap();
            let image_view = device
                .create_image_view(
                    &image,
                    i::ViewKind::D2,
                    ScreenBufferFormat::SELF,
                    Swizzle::NO,
                    COLOR_RANGE.clone(),
                )
                .unwrap();

            let sampler = device
                .create_sampler(i::SamplerInfo::new(i::Filter::Nearest, i::WrapMode::Clamp))
                .expect("Can't create sampler");

            desc.write_to_state(
                vec![
                    DescSetWrite {
                        binding: 0,
                        array_offset: 0,
                        descriptors: Some(pso::Descriptor::Image(
                            &image_view,
                            i::Layout::Undefined,
                        )),
                    },
                    DescSetWrite {
                        binding: 1,
                        array_offset: 0,
                        descriptors: Some(pso::Descriptor::Sampler(&sampler)),
                    },
                ],
                device,
            );

            let transferred_image_fence = device.create_fence(false).expect("Can't create fence");
            (
                desc,
                Some(staging_buffer),
                Some(staging_buffer_memory),
                staging_buffer_memory_requirements,
                Some(sampler),
                Some(image_view),
                Some(image),
                Some(texture_memory),
                Some(transferred_image_fence),
            )
        };

        NesScreenBuffer {
            device: device_ptr,
            desc,
            staging_buffer,
            staging_buffer_memory,
            sampler,
            image_view,
            image,
            texture_memory,
            image_transfer_fence,
            dimensions: (image_width, image_height),
            row_pitch,
            stride,
            row_alignment_mask,
            staging_buffer_size: staging_buffer_memory_requirements.size,
        }
    }

    pub fn descriptor_set(&self) -> &B::DescriptorSet {
        self.desc
            .set
            .as_ref()
            .expect("Unable to retrieve screen buffer descriptor set")
    }

    pub fn update_buffer_data(&mut self, data_source: &[u8]) {
        let device = &self.device.borrow().device;
        let upload_size = data_source.len() as u64;
        let (width, height) = self.dimensions;
        let row_pitch =
            (width * self.stride as u32 + self.row_alignment_mask) & !self.row_alignment_mask;
        assert!(upload_size <= self.staging_buffer_size);
        unsafe {
            let mut data_target = device
                .acquire_mapping_writer::<u8>(
                    self.staging_buffer_memory.as_ref().unwrap(),
                    0..self.staging_buffer_size,
                )
                .unwrap();
            for y in 0..height as usize {
                let row = &(*data_source)[y * (width as usize) * (self.stride as usize)
                    ..(y + 1) * (width as usize) * (self.stride as usize)];
                let dest_base = y * row_pitch as usize;
                data_target[dest_base..dest_base + row.len()].copy_from_slice(row);
            }
            device.release_mapping_writer(data_target).unwrap();
        }
    }

    pub fn copy_buffer_to_texture(
        &self,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut CommandPool<B, Graphics>,
    ) {
        let (image_width, image_height) = self.dimensions;
        let mut cmd_buffer = staging_pool.acquire_command_buffer::<command::OneShot>();

        unsafe {
            cmd_buffer.begin();
        }

        let image_barrier = m::Barrier::Image {
            states: (i::Access::empty(), i::Layout::Undefined)
                ..(i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal),
            target: self.image.as_ref().unwrap(),
            families: None,
            range: COLOR_RANGE.clone(),
        };

        unsafe {
            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.copy_buffer_to_image(
                self.staging_buffer.as_ref().unwrap(),
                self.image.as_ref().unwrap(),
                i::Layout::TransferDstOptimal,
                &[command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: self.row_pitch / (self.stride as u32),
                    buffer_height: image_height as u32,
                    image_layers: i::SubresourceLayers {
                        aspects: f::Aspects::COLOR,
                        level: 0,
                        layers: 0..1,
                    },
                    image_offset: i::Offset { x: 0, y: 0, z: 0 },
                    image_extent: i::Extent {
                        width: image_width,
                        height: image_height,
                        depth: 1,
                    },
                }],
            );
        }

        let image_barrier = m::Barrier::Image {
            states: (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal)
                ..(i::Access::SHADER_READ, i::Layout::ShaderReadOnlyOptimal),
            target: self.image.as_ref().unwrap(),
            families: None,
            range: COLOR_RANGE.clone(),
        };

        unsafe {
            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.finish();

            device_state.queues.queues[0]
                .submit_nosemaphores(iter::once(&cmd_buffer), self.image_transfer_fence.as_ref());
        }
    }

    pub fn wait_for_transfer_completion(&self) {
        let device = &self.desc.layout.device.borrow().device;
        unsafe {
            let fence = self.image_transfer_fence.as_ref().unwrap();
            device.wait_for_fence(fence, !0).unwrap();

            device
                .reset_fence(fence)
                .expect("Fence to reset successfully");
        }
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.get_layout()
    }
}

impl<B: Backend> Drop for NesScreenBuffer<B> {
    fn drop(&mut self) {
        let device = &self.desc.layout.device.borrow().device;
        let fence = self
            .image_transfer_fence
            .take()
            .expect("Fence shouldn't be None");
        let wait_timeout_ns = 10_000;

        unsafe {
            device
                .wait_for_fence(&fence, wait_timeout_ns)
                .expect("Image transfer fence shouldn't timeout");
            device.destroy_fence(fence);
            device.destroy_sampler(self.sampler.take().expect("Unable to destroy sampler"));
            device.destroy_image_view(
                self.image_view
                    .take()
                    .expect("Unable to destroy image view"),
            );
            device.destroy_image(self.image.take().expect("Unable to destroy image"));
            device.free_memory(
                self.texture_memory
                    .take()
                    .expect("Unable to free texture memory"),
            );
            device.destroy_buffer(
                self.staging_buffer
                    .take()
                    .expect("Unable to destroy staging buffer"),
            );
            device.free_memory(
                self.staging_buffer_memory
                    .take()
                    .expect("Unable to free staging buffer memory"),
            );
        }
    }
}
