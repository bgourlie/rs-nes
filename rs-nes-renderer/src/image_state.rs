use std::{iter, rc::Rc};

use hal::{
    buffer, command, format as f,
    format::{AsFormat, Rgba8Srgb as ColorFormat, Swizzle},
    image as i, memory as m,
    pso::{self, PipelineStage},
    Backend, Device,
};

use crate::{
    adapter_state::AdapterState,
    buffer_state::BufferState,
    descriptor_set::{DescSet, DescSetWrite},
    device_state::DeviceState,
    dimensions::Dimensions,
    BYTES_PER_PIXEL, COLOR_RANGE, IMAGE_HEIGHT, IMAGE_WIDTH,
};

pub struct ImageState<B: Backend> {
    pub desc: DescSet<B>,
    buffer: Option<BufferState<B>>,
    sampler: Option<B::Sampler>,
    image_view: Option<B::ImageView>,
    image: Option<B::Image>,
    memory: Option<B::Memory>,
    transferred_image_fence: Option<B::Fence>,
    screen_buffer: Vec<u8>,
    dimensions: Dimensions<u32>,
    row_pitch: u32,
    stride: usize,
}

impl<B: Backend> ImageState<B> {
    pub unsafe fn new<T: ::hal::Supports<::hal::Transfer>>(
        mut desc: DescSet<B>,
        adapter: &AdapterState<B>,
        usage: buffer::Usage,
        device_state: &mut DeviceState<B>,
    ) -> Self {
        let screen_buffer = vec![255_u8; IMAGE_WIDTH * IMAGE_HEIGHT * BYTES_PER_PIXEL];
        let (buffer, dimensions, row_pitch, stride) = BufferState::new_texture(
            Rc::clone(&desc.layout.device),
            &device_state.device,
            &screen_buffer,
            adapter,
            usage,
        );

        let buffer = Some(buffer);
        let device = &mut device_state.device;

        let kind = i::Kind::D2(
            dimensions.width as i::Size,
            dimensions.height as i::Size,
            1,
            1,
        );
        let mut image = device
            .create_image(
                kind,
                1,
                ColorFormat::SELF,
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

        let memory = device.allocate_memory(device_type, req.size).unwrap();

        device.bind_image_memory(&memory, 0, &mut image).unwrap();
        let image_view = device
            .create_image_view(
                &image,
                i::ViewKind::D2,
                ColorFormat::SELF,
                Swizzle::NO,
                COLOR_RANGE.clone(),
            )
            .unwrap();

        let sampler = device
            .create_sampler(i::SamplerInfo::new(i::Filter::Linear, i::WrapMode::Clamp))
            .expect("Can't create sampler");

        desc.write_to_state(
            vec![
                DescSetWrite {
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Image(&image_view, i::Layout::Undefined)),
                },
                DescSetWrite {
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Sampler(&sampler)),
                },
            ],
            device,
        );

        let transfered_image_fence = device.create_fence(false).expect("Can't create fence");

        ImageState {
            desc,
            buffer,
            sampler: Some(sampler),
            image_view: Some(image_view),
            image: Some(image),
            memory: Some(memory),
            transferred_image_fence: Some(transfered_image_fence),
            screen_buffer,
            dimensions,
            row_pitch,
            stride,
        }
    }

    pub fn update_screen_buffer(&mut self, t: std::time::Duration) {
        let t_mod = (t.as_secs() % 255) as usize + 1;
        for y in 0..IMAGE_HEIGHT {
            for x in 0..IMAGE_WIDTH {
                let i = (y * IMAGE_WIDTH + x) * BYTES_PER_PIXEL;
                self.screen_buffer[i] = (y % t_mod) as u8;
                self.screen_buffer[i + 1] = x as u8;
                self.screen_buffer[i + 2] = ((x + y) % 255) as u8;
            }
        }

        self.buffer
            .as_mut()
            .unwrap()
            .update_data(0, &self.screen_buffer);
    }

    pub unsafe fn copy_buffer_to_texture(
        &self,
        device_state: &mut DeviceState<B>,
        staging_pool: &mut ::hal::CommandPool<B, ::hal::Graphics>,
    ) {
        let mut cmd_buffer = staging_pool.acquire_command_buffer::<command::OneShot>();
        cmd_buffer.begin();

        let image_barrier = m::Barrier::Image {
            states: (i::Access::empty(), i::Layout::Undefined)
                ..(i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal),
            target: self.image.as_ref().unwrap(),
            families: None,
            range: COLOR_RANGE.clone(),
        };

        cmd_buffer.pipeline_barrier(
            PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
            m::Dependencies::empty(),
            &[image_barrier],
        );

        cmd_buffer.copy_buffer_to_image(
            self.buffer.as_ref().unwrap().get_buffer(),
            self.image.as_ref().unwrap(),
            i::Layout::TransferDstOptimal,
            &[command::BufferImageCopy {
                buffer_offset: 0,
                buffer_width: self.row_pitch / (self.stride as u32),
                buffer_height: self.dimensions.height as u32,
                image_layers: i::SubresourceLayers {
                    aspects: f::Aspects::COLOR,
                    level: 0,
                    layers: 0..1,
                },
                image_offset: i::Offset { x: 0, y: 0, z: 0 },
                image_extent: i::Extent {
                    width: self.dimensions.width,
                    height: self.dimensions.height,
                    depth: 1,
                },
            }],
        );

        let image_barrier = m::Barrier::Image {
            states: (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal)
                ..(i::Access::SHADER_READ, i::Layout::ShaderReadOnlyOptimal),
            target: self.image.as_ref().unwrap(),
            families: None,
            range: COLOR_RANGE.clone(),
        };
        cmd_buffer.pipeline_barrier(
            PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
            m::Dependencies::empty(),
            &[image_barrier],
        );

        cmd_buffer.finish();

        device_state.queues.queues[0].submit_nosemaphores(
            iter::once(&cmd_buffer),
            self.transferred_image_fence.as_ref(),
        );
    }

    pub fn wait_for_transfer_completion(&self) {
        let device = &self.desc.layout.device.borrow().device;
        unsafe {
            device
                .wait_for_fence(self.transferred_image_fence.as_ref().unwrap(), !0)
                .unwrap();
        }
    }

    pub fn get_layout(&self) -> &B::DescriptorSetLayout {
        self.desc.get_layout()
    }
}

impl<B: Backend> Drop for ImageState<B> {
    fn drop(&mut self) {
        unsafe {
            let device = &self.desc.layout.device.borrow().device;

            let fence = self.transferred_image_fence.take().unwrap();
            device.wait_for_fence(&fence, !0).unwrap();
            device.destroy_fence(fence);

            device.destroy_sampler(self.sampler.take().unwrap());
            device.destroy_image_view(self.image_view.take().unwrap());
            device.destroy_image(self.image.take().unwrap());
            device.free_memory(self.memory.take().unwrap());
        }

        self.buffer.take().unwrap();
    }
}
