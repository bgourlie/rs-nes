use gfx_hal::{
    buffer,
    command::{self, CommandBuffer},
    format, image, memory,
    pso::{self, PipelineStage},
    Backend, Device, Graphics, Limits, MemoryType,
};

use crate::{
    descriptor_set::{DescSet, DescSetWrite},
    ScreenBufferFormat, COLOR_RANGE,
};

use rs_nes::{PPU_BUFFER_SIZE, PPU_PIXEL_STRIDE};

pub struct NesScreen<B: Backend> {
    desc: DescSet<B>,
    staging_buffer: B::Buffer,
    staging_buffer_memory: B::Memory,
    sampler: B::Sampler,
    image_view: B::ImageView,
    image: B::Image,
    texture_memory: B::Memory,
    dimensions: (u32, u32),
    row_pitch: u32,
    row_alignment_mask: u32,
    staging_buffer_size: u64,
}

impl<B: Backend> NesScreen<B> {
    pub fn new(
        device: &mut B::Device,
        width: u32,
        height: u32,
        mut desc: DescSet<B>,
        limits: Limits,
        memory_types: &[MemoryType],
    ) -> Self {
        let row_alignment_mask = limits.min_buffer_copy_pitch_alignment as u32 - 1;
        let row_pitch =
            (width * PPU_PIXEL_STRIDE as u32 + row_alignment_mask) & !row_alignment_mask;
        let upload_size = u64::from(height * row_pitch);
        println!("Row alignment mask: {}", row_alignment_mask);

        let image_kind = image::Kind::D2(width as image::Size, height as image::Size, 1, 1);
        let (mut staging_buffer, staging_buffer_memory_requirements) = unsafe {
            let staging_buffer = device
                .create_buffer(upload_size, buffer::Usage::TRANSFER_SRC)
                .expect("Unable to create staging buffer");

            let staging_buffer_memory_requirements =
                device.get_buffer_requirements(&staging_buffer);

            (staging_buffer, staging_buffer_memory_requirements)
        };

        let staging_buffer_memory_type = memory_types
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

        let texture_memory_type = memory_types
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

        NesScreen {
            desc,
            staging_buffer,
            staging_buffer_memory,
            sampler,
            image_view,
            image,
            texture_memory,
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

    pub fn update_buffer_data(&mut self, data_source: &[u8; PPU_BUFFER_SIZE], device: &B::Device) {
        let upload_size = data_source.len() as u64;
        let (width, height) = self.dimensions;
        let row_pitch =
            (width * PPU_PIXEL_STRIDE as u32 + self.row_alignment_mask) & !self.row_alignment_mask;
        debug_assert!(upload_size <= self.staging_buffer_size);

        unsafe {
            let mut data_target = device
                .acquire_mapping_writer::<u8>(
                    &self.staging_buffer_memory,
                    0..self.staging_buffer_size,
                )
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

    pub fn record_transfer_commands(&self, command_buffer: &mut CommandBuffer<B, Graphics>) {
        let (image_width, image_height) = self.dimensions;

        let image_barrier = memory::Barrier::Image {
            states: (image::Access::empty(), image::Layout::Undefined)
                ..(
                    image::Access::TRANSFER_WRITE,
                    image::Layout::TransferDstOptimal,
                ),
            target: &self.image,
            families: None,
            range: COLOR_RANGE.clone(),
        };

        unsafe {
            command_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                memory::Dependencies::empty(),
                &[image_barrier],
            );

            command_buffer.copy_buffer_to_image(
                &self.staging_buffer,
                &self.image,
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
            target: &self.image,
            families: None,
            range: COLOR_RANGE.clone(),
        };

        unsafe {
            command_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                memory::Dependencies::empty(),
                &[image_barrier],
            );
        }
    }

    pub fn layout(&self) -> &B::DescriptorSetLayout {
        self.desc.layout()
    }

    pub fn destroy(self, device: &B::Device) {
        unsafe {
            device.destroy_sampler(self.sampler);
            device.destroy_image_view(self.image_view);
            device.destroy_image(self.image);
            device.free_memory(self.texture_memory);
            device.destroy_buffer(self.staging_buffer);
            device.free_memory(self.staging_buffer_memory);
            self.desc.destroy(device);
        }
    }
}
