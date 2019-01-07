use std::{cell::RefCell, iter, rc::Rc};

use gfx_hal::{
    buffer, command, format, image, memory,
    pso::{self, PipelineStage},
    Backend, CommandPool, Device, Graphics, Supports, Transfer,
};

use crate::{
    adapter_state::AdapterState,
    descriptor_set::{DescSet, DescSetWrite},
    device_state::DeviceState,
    ScreenBufferFormat, COLOR_RANGE,
};

use rs_nes::{PPU_BUFFER_SIZE, PPU_PIXEL_STRIDE};

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
    staging_buffer_size: u64,
}

impl<B: Backend> NesScreenBuffer<B> {
    pub fn new<T: Supports<Transfer>>(
        device_ptr: Rc<RefCell<DeviceState<B>>>,
        width: u32,
        height: u32,
        mut desc: DescSet<B>,
        adapter: &AdapterState<B>,
    ) -> Self {
        let row_alignment_mask = adapter.limits.min_buffer_copy_pitch_alignment as u32 - 1;
        let row_pitch =
            (width * PPU_PIXEL_STRIDE as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = u64::from(height * row_pitch);
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
            let image_kind = image::Kind::D2(width as image::Size, height as image::Size, 1, 1);

            let device = &mut device_ptr.borrow_mut().device;

            let (mut staging_buffer, staging_buffer_memory_requirements) = unsafe {
                let staging_buffer = device
                    .create_buffer(upload_size, buffer::Usage::TRANSFER_SRC)
                    .expect("Unable to create staging buffer");

                let staging_buffer_memory_requirements =
                    device.get_buffer_requirements(&staging_buffer);

                (staging_buffer, staging_buffer_memory_requirements)
            };

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
                .expect("Staging buffer memory type not supported")
                .into();

            let (staging_buffer_memory, mut image, req) = unsafe {
                let staging_buffer_memory = device
                    .allocate_memory(
                        staging_buffer_memory_type,
                        staging_buffer_memory_requirements.size,
                    )
                    .expect("Unable to allocate staging buffer memory");

                device
                    .bind_buffer_memory(&staging_buffer_memory, 0, &mut staging_buffer)
                    .expect("Unable to bind staging buffer memory");

                let image = device
                    .create_image(
                        image_kind,
                        1,
                        <ScreenBufferFormat as format::AsFormat>::SELF,
                        image::Tiling::Optimal,
                        image::Usage::TRANSFER_DST | image::Usage::SAMPLED,
                        image::ViewCapabilities::empty(),
                    )
                    .expect("Unable to create image");

                let req = device.get_image_requirements(&image);
                (staging_buffer_memory, image, req)
            };

            let texture_memory_type = adapter
                .memory_types
                .iter()
                .enumerate()
                .position(|(id, memory_type)| {
                    req.type_mask & (1 << id) != 0
                        && memory_type
                            .properties
                            .contains(memory::Properties::DEVICE_LOCAL)
                })
                .expect("Texture memory type not supported")
                .into();

            let (texture_memory, image_view, sampler) = unsafe {
                let texture_memory = device
                    .allocate_memory(texture_memory_type, req.size)
                    .expect("Unable to allocate texture memory");

                device
                    .bind_image_memory(&texture_memory, 0, &mut image)
                    .expect("Unable to bind texture memory to image");

                let image_view = device
                    .create_image_view(
                        &image,
                        image::ViewKind::D2,
                        <ScreenBufferFormat as format::AsFormat>::SELF,
                        format::Swizzle::NO,
                        COLOR_RANGE.clone(),
                    )
                    .expect("Unable to create image view");

                let sampler = device
                    .create_sampler(image::SamplerInfo::new(
                        image::Filter::Nearest,
                        image::WrapMode::Clamp,
                    ))
                    .expect("Can't create sampler");

                desc.write_to_state(
                    vec![
                        DescSetWrite {
                            binding: 0,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Image(
                                &image_view,
                                image::Layout::Undefined,
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

                (texture_memory, image_view, sampler)
            };

            let image_transfer_fence = device.create_fence(false).expect("Can't create fence");
            (
                desc,
                Some(staging_buffer),
                Some(staging_buffer_memory),
                staging_buffer_memory_requirements,
                Some(sampler),
                Some(image_view),
                Some(image),
                Some(texture_memory),
                Some(image_transfer_fence),
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
            dimensions: (width, height),
            row_pitch,
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

    pub fn update_buffer_data(&mut self, data_source: &[u8; PPU_BUFFER_SIZE]) {
        let device = &self.device.borrow().device;
        let upload_size = data_source.len() as u64;
        let (width, height) = self.dimensions;
        let row_pitch =
            (width * PPU_PIXEL_STRIDE as u32 + self.row_alignment_mask) & !self.row_alignment_mask;
        debug_assert!(upload_size <= self.staging_buffer_size);

        unsafe {
            let staging_buffer_memory = self
                .staging_buffer_memory
                .as_ref()
                .expect("Staging buffer memory should't be None");

            let mut data_target = device
                .acquire_mapping_writer::<u8>(staging_buffer_memory, 0..self.staging_buffer_size)
                .expect("Unable to acquire staging buffer mapping writer");

            for y in 0..height as usize {
                let width = width as usize;
                let row_start = y * width * PPU_PIXEL_STRIDE;
                let row_end = (y + 1) * width * PPU_PIXEL_STRIDE;
                let row = &(*data_source)[row_start..row_end];
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

        let image_barrier = memory::Barrier::Image {
            states: (image::Access::empty(), image::Layout::Undefined)
                ..(
                    image::Access::TRANSFER_WRITE,
                    image::Layout::TransferDstOptimal,
                ),
            target: self.image.as_ref().unwrap(),
            families: None,
            range: COLOR_RANGE.clone(),
        };

        unsafe {
            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                memory::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.copy_buffer_to_image(
                self.staging_buffer.as_ref().unwrap(),
                self.image.as_ref().unwrap(),
                image::Layout::TransferDstOptimal,
                &[command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: self.row_pitch / (PPU_PIXEL_STRIDE as u32),
                    buffer_height: image_height as u32,
                    image_layers: image::SubresourceLayers {
                        aspects: format::Aspects::COLOR,
                        level: 0,
                        layers: 0..1,
                    },
                    image_offset: image::Offset { x: 0, y: 0, z: 0 },
                    image_extent: image::Extent {
                        width: image_width,
                        height: image_height,
                        depth: 1,
                    },
                }],
            );
        }

        let image_barrier = memory::Barrier::Image {
            states: (
                image::Access::TRANSFER_WRITE,
                image::Layout::TransferDstOptimal,
            )
                ..(
                    image::Access::SHADER_READ,
                    image::Layout::ShaderReadOnlyOptimal,
                ),
            target: self.image.as_ref().unwrap(),
            families: None,
            range: COLOR_RANGE.clone(),
        };

        unsafe {
            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                memory::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.finish();

            device_state.queues.queues[0]
                .submit_nosemaphores(iter::once(&cmd_buffer), self.image_transfer_fence.as_ref());
        }
    }

    pub fn wait_for_transfer_completion(&self, device: &B::Device) {
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
        let device = &self.device.borrow().device;
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

            let descriptor_set_layout = self.desc.take_resources();
            device.destroy_descriptor_set_layout(descriptor_set_layout);
        }
    }
}
