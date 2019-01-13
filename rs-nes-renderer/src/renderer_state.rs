use std::{iter, mem::size_of};

use gfx_hal::{
    buffer, command, memory,
    pso::{self, PipelineStage, ShaderStageFlags},
    queue::Submission,
    Backend, Device, FrameSync, Graphics, SwapImageIndex, Swapchain,
};

use crate::{
    adapter_state::AdapterState, backend_state::BackendState, descriptor_set::DescSetLayout,
    device_state::DeviceState, framebuffer_state::FramebufferState, nes_screen::NesScreen,
    palette::PALETTE, palette_uniform::PaletteUniform, pipeline_state::PipelineState,
    render_pass_state::RenderPassState, swapchain_state::SwapchainState, vertex::Vertex,
    COLOR_RANGE, DIMS, QUAD,
};

use winit::Window;

use rs_nes::{PPU_BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

pub enum RenderStatus {
    Normal,
    NormalAndSwapchainRecreated,
    RecreateSwapchain,
}

pub struct RendererState<B: Backend> {
    pub surface: B::Surface,
    pub adapter: AdapterState<B>,
    pub window: Option<Window>,
    uniform_desc_pool: Option<B::DescriptorPool>,
    img_desc_pool: Option<B::DescriptorPool>,
    swapchain: SwapchainState<B>,
    device: DeviceState<B>,
    vertex_memory: Option<B::Memory>,
    vertex_buffer: Option<B::Buffer>,
    render_pass: RenderPassState<B>,
    palette_uniform: PaletteUniform<B>,
    pipeline: PipelineState<B>,
    framebuffer: FramebufferState<B>,
    viewport: pso::Viewport,
    nes_screen: NesScreen<B>,
}

impl<B: Backend> RendererState<B> {
    pub fn new(mut backend: BackendState<B>) -> Self {
        let (mut surface, mut adapter, window) = {
            let surface = backend.surface.take().expect("Surface shouldn't be None");
            let adapter = backend.adapter.take().expect("Adapter shouldn't be None");
            let window = backend.window.take();

            if !is_gl_backend() && window.is_none() {
                panic!("Window shouldn't be None")
            }

            (surface, adapter, window)
        };

        let mut device = DeviceState::new(adapter.adapter.take().unwrap(), &surface);

        let image_desc = DescSetLayout::new(
            &device.device,
            vec![
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
        );

        let uniform_desc = DescSetLayout::new(
            &device.device,
            vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }],
        );

        let mut img_desc_pool = unsafe {
            device
                .device
                .create_descriptor_pool(
                    1, // # of sets
                    &[
                        pso::DescriptorRangeDesc {
                            ty: pso::DescriptorType::SampledImage,
                            count: 1,
                        },
                        pso::DescriptorRangeDesc {
                            ty: pso::DescriptorType::Sampler,
                            count: 1,
                        },
                    ],
                )
                .ok()
        };

        let mut uniform_desc_pool = unsafe {
            device
                .device
                .create_descriptor_pool(
                    1, // # of sets
                    &[pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBuffer,
                        count: 1,
                    }],
                )
                .ok()
        };

        let (image_desc, uniform_desc) = unsafe {
            (
                image_desc.create_desc_set(img_desc_pool.as_mut().unwrap()),
                uniform_desc.create_desc_set(uniform_desc_pool.as_mut().unwrap()),
            )
        };

        let nes_screen_buffer = NesScreen::new::<Graphics>(
            &mut device,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            image_desc,
            &adapter,
        );

        let (vertex_memory, vertex_buffer) = {
            let vertex_stride = size_of::<Vertex>() as u64;
            let vertex_upload_size = QUAD.len() as u64 * vertex_stride;

            let device = &device.device;

            let mut buffer = unsafe {
                device
                    .create_buffer(vertex_upload_size, buffer::Usage::VERTEX)
                    .unwrap()
            };

            let mem_req = unsafe { device.get_buffer_requirements(&buffer) };

            let memory_type = &adapter
                .memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type
                            .properties
                            .contains(memory::Properties::CPU_VISIBLE)
                })
                .expect("Vertex memory type not supported")
                .into();

            unsafe {
                let memory = device.allocate_memory(*memory_type, mem_req.size).unwrap();
                device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();

                let mut data_target = device
                    .acquire_mapping_writer::<Vertex>(&memory, 0..mem_req.size)
                    .expect("Unable to acquire mapping writer");

                data_target[0..QUAD.len()].copy_from_slice(&QUAD);

                device
                    .release_mapping_writer(data_target)
                    .expect("Unable to release mapping writer");
                (memory, buffer)
            }
        };

        let palette_uniform = unsafe {
            PaletteUniform::new(
                &mut device.device,
                &adapter.memory_types,
                &PALETTE,
                uniform_desc,
            )
        };

        let (swapchain, render_pass, framebuffer, pipeline) = unsafe {
            let mut swapchain = SwapchainState::new(&mut surface, &device, DIMS);
            let render_pass = RenderPassState::new(&swapchain, &device.device);

            let framebuffer =
                FramebufferState::new(&device, &render_pass, &mut swapchain, &COLOR_RANGE);

            let pipeline = PipelineState::new(
                vec![nes_screen_buffer.layout(), palette_uniform.layout()],
                render_pass.render_pass.as_ref().unwrap(),
                &device.device,
            );

            (swapchain, render_pass, framebuffer, pipeline)
        };

        let viewport = RendererState::create_viewport(&swapchain);

        RendererState {
            adapter,
            surface,
            window,
            device,
            nes_screen: nes_screen_buffer,
            img_desc_pool,
            uniform_desc_pool,
            vertex_buffer: Some(vertex_buffer),
            vertex_memory: Some(vertex_memory),
            palette_uniform,
            render_pass,
            pipeline,
            swapchain,
            framebuffer,
            viewport,
        }
    }

    fn recreate_swapchain(&mut self) {
        self.device.device.wait_idle().unwrap();

        unsafe {
            SwapchainState::destroy_resources(&mut self.swapchain, &self.device.device);
            self.swapchain = SwapchainState::new(&mut self.surface, &self.device, DIMS);

            RenderPassState::destroy_resources(&mut self.render_pass, &self.device.device);
            self.render_pass = RenderPassState::new(&self.swapchain, &self.device.device);

            FramebufferState::destroy_resources(&mut self.framebuffer, &self.device.device);
            self.framebuffer = FramebufferState::new(
                &self.device,
                &self.render_pass,
                &mut self.swapchain,
                &COLOR_RANGE,
            );

            PipelineState::destroy_resources(&mut self.pipeline, &self.device.device);
            self.pipeline = PipelineState::new(
                vec![self.nes_screen.layout(), self.palette_uniform.layout()],
                self.render_pass.render_pass.as_ref().unwrap(),
                &self.device.device,
            );
        }

        self.viewport = RendererState::create_viewport(&self.swapchain);
    }

    fn create_viewport(swapchain: &SwapchainState<B>) -> pso::Viewport {
        pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: swapchain.extent.width as i16,
                h: swapchain.extent.height as i16,
            },
            depth: 0.0..1.0,
        }
    }

    pub fn render_frame(
        &mut self,
        screen_buffer: &[u8; PPU_BUFFER_SIZE],
        recreate_swapchain: bool,
    ) -> RenderStatus {
        let mut render_status = if recreate_swapchain {
            self.recreate_swapchain();
            RenderStatus::NormalAndSwapchainRecreated
        } else {
            RenderStatus::Normal
        };

        self.nes_screen
            .update_buffer_data(screen_buffer, &self.device.device);

        // The following line causing huge memory leak with dx12 backend
        // See https://github.com/gfx-rs/gfx/issues/2556
        // TODO: Refactor so that the buffer copy reuses command buffer instead of creating its own
        self.nes_screen.copy_buffer_to_texture(&mut self.device);

        self.nes_screen
            .wait_for_transfer_completion(&self.device.device);

        let semaphore_index = self.framebuffer.next_acq_pre_pair_index();

        let frame: SwapImageIndex = unsafe {
            let (acquire_semaphore, _) = self.framebuffer.get_frame_data(None, semaphore_index).1;
            match self
                .swapchain
                .swapchain
                .as_mut()
                .expect("Swapchain shouldn't be None")
                .acquire_image(!0, FrameSync::Semaphore(acquire_semaphore))
            {
                Ok(image_index) => image_index,
                Err(_) => return RenderStatus::RecreateSwapchain,
            }
        };

        let (frame_data, framebuffer_semaphores) = self
            .framebuffer
            .get_frame_data(Some(frame as usize), semaphore_index);

        let (framebuffer_fence, framebuffer, command_pool) = frame_data.unwrap();
        let (image_acquired_semaphore, image_present_semaphores) = framebuffer_semaphores;

        unsafe {
            self.device
                .device
                .wait_for_fence(framebuffer_fence, !0)
                .unwrap();
            self.device.device.reset_fence(framebuffer_fence).unwrap();
            command_pool.reset();

            // Rendering
            let mut cmd_buffer = command_pool.acquire_command_buffer::<command::OneShot>();
            cmd_buffer.begin();

            cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
            cmd_buffer.set_scissors(0, &[self.viewport.rect]);
            cmd_buffer.bind_graphics_pipeline(self.pipeline.pipeline.as_ref().unwrap());
            cmd_buffer.bind_vertex_buffers(
                0,
                Some((
                    self.vertex_buffer
                        .as_ref()
                        .expect("Vertex buffer shouldn't be None"),
                    0,
                )),
            );
            cmd_buffer.bind_graphics_descriptor_sets(
                self.pipeline.pipeline_layout.as_ref().unwrap(),
                0,
                vec![
                    self.nes_screen.descriptor_set(),
                    self.palette_uniform.descriptor_set(),
                ],
                &[],
            );

            {
                let mut encoder = cmd_buffer.begin_render_pass_inline(
                    self.render_pass.render_pass.as_ref().unwrap(),
                    framebuffer,
                    self.viewport.rect,
                    &[command::ClearValue::Color(command::ClearColor::Float([
                        0.8, 0.8, 0.8, 1.0,
                    ]))],
                );
                encoder.draw(0..6, 0..1);
            }
            cmd_buffer.finish();

            let submission = Submission {
                command_buffers: iter::once(&cmd_buffer),
                wait_semaphores: iter::once((
                    &*image_acquired_semaphore,
                    PipelineStage::BOTTOM_OF_PIPE,
                )),
                signal_semaphores: iter::once(&*image_present_semaphores),
            };

            self.device.queues.queues[0].submit(submission, Some(framebuffer_fence));

            // present frame
            if self
                .swapchain
                .swapchain
                .as_ref()
                .unwrap()
                .present(
                    &mut self.device.queues.queues[0],
                    frame,
                    Some(&*image_present_semaphores),
                )
                .is_err()
            {
                render_status = RenderStatus::RecreateSwapchain
            }
        }

        render_status
    }
}

impl<B: Backend> Drop for RendererState<B> {
    fn drop(&mut self) {
        self.device.device.wait_idle().unwrap();
        unsafe {
            self.device.device.destroy_descriptor_pool(
                self.img_desc_pool
                    .take()
                    .expect("Image descriptor pool shouldn't be None"),
            );

            self.device.device.destroy_descriptor_pool(
                self.uniform_desc_pool
                    .take()
                    .expect("Uniform descriptor pool shouldn't be None"),
            );

            self.device.device.destroy_buffer(
                self.vertex_buffer
                    .take()
                    .expect("Vertex buffer shouldn't be None"),
            );

            self.device.device.free_memory(
                self.vertex_memory
                    .take()
                    .expect("Vertex memory shouldn't be None"),
            );
        }

        PaletteUniform::destroy_resources(&mut self.palette_uniform, &self.device.device);
        NesScreen::destroy_resources(&mut self.nes_screen, &self.device.device);
        FramebufferState::destroy_resources(&mut self.framebuffer, &self.device.device);
        SwapchainState::destroy_resources(&mut self.swapchain, &self.device.device);
        PipelineState::destroy_resources(&mut self.pipeline, &self.device.device);
        RenderPassState::destroy_resources(&mut self.render_pass, &self.device.device);
    }
}

const fn is_gl_backend() -> bool {
    cfg!(feature = "gl")
}
