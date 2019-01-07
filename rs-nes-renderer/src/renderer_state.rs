use std::{cell::RefCell, iter, mem::size_of, rc::Rc};

use gfx_hal::{
    buffer, command, memory,
    pso::{self, PipelineStage, ShaderStageFlags},
    queue::Submission,
    Backend, Device, FrameSync, Graphics, SwapImageIndex, Swapchain,
};

use crate::{
    backend_state::BackendState, descriptor_set::DescSetLayout, device_state::DeviceState,
    framebuffer_state::FramebufferState, nes_screen_buffer::NesScreenBuffer,
    pipeline_state::PipelineState, render_pass_state::RenderPassState,
    swapchain_state::SwapchainState, uniform::Uniform, vertex::Vertex, COLOR_RANGE, DIMS, QUAD,
};

use rs_nes::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub enum RenderStatus {
    Normal,
    NormalAndSwapchainRecreated,
    RecreateSwapchain,
}

pub struct RendererState<B: Backend> {
    uniform_desc_pool: Option<B::DescriptorPool>,
    img_desc_pool: Option<B::DescriptorPool>,
    swapchain: Option<SwapchainState<B>>,
    pub device: Rc<RefCell<DeviceState<B>>>,
    pub backend: BackendState<B>,
    vertex_memory: Option<B::Memory>,
    vertex_buffer: Option<B::Buffer>,
    render_pass: RenderPassState<B>,
    uniform: Uniform<B>,
    pipeline: PipelineState<B>,
    framebuffer: FramebufferState<B>,
    viewport: pso::Viewport,
    pub nes_screen_buffer: NesScreenBuffer<B>,
}

impl<B: Backend> RendererState<B> {
    pub unsafe fn new(mut backend: BackendState<B>) -> Self {
        let device = Rc::new(RefCell::new(DeviceState::new(
            backend.adapter.adapter.take().unwrap(),
            &backend.surface,
        )));

        let image_desc = DescSetLayout::new(
            Rc::clone(&device),
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
            Rc::clone(&device),
            vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }],
        );

        let mut img_desc_pool = device
            .borrow()
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
            .ok();

        let mut uniform_desc_pool = device
            .borrow()
            .device
            .create_descriptor_pool(
                1, // # of sets
                &[pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::UniformBuffer,
                    count: 1,
                }],
            )
            .ok();

        let image_desc = image_desc.create_desc_set(img_desc_pool.as_mut().unwrap());
        let uniform_desc = uniform_desc.create_desc_set(uniform_desc_pool.as_mut().unwrap());

        let nes_screen_buffer = NesScreenBuffer::new::<Graphics>(
            device.clone(),
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            image_desc,
            &backend.adapter,
        );

        let (vertex_memory, vertex_buffer) = {
            let vertex_stride = size_of::<Vertex>() as u64;
            let vertex_upload_size = QUAD.len() as u64 * vertex_stride;

            let device = &device.borrow().device;

            let mut buffer = device
                .create_buffer(vertex_upload_size, buffer::Usage::VERTEX)
                .unwrap();
            let mem_req = device.get_buffer_requirements(&buffer);

            let memory_type = &backend
                .adapter
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

            let memory = device.allocate_memory(*memory_type, mem_req.size).unwrap();
            device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
            let size = mem_req.size;

            let mut data_target = device
                .acquire_mapping_writer::<Vertex>(&memory, 0..size)
                .expect("Unable to acquire mapping writer");

            data_target[0..QUAD.len()].copy_from_slice(&QUAD);

            device
                .release_mapping_writer(data_target)
                .expect("Unable to release mapping writer");

            (memory, buffer)
        };

        let uniform = Uniform::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &[1f32, 1.0f32, 1.0f32, 1.0f32],
            uniform_desc,
            0,
        );

        let mut swapchain = Some(SwapchainState::new(&mut backend, Rc::clone(&device), DIMS));

        let render_pass = RenderPassState::new(swapchain.as_ref().unwrap(), Rc::clone(&device));

        let framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &render_pass,
            swapchain.as_mut().unwrap(),
            &COLOR_RANGE,
        );

        let pipeline = PipelineState::new(
            vec![nes_screen_buffer.get_layout(), uniform.layout()],
            render_pass.render_pass.as_ref().unwrap(),
            Rc::clone(&device),
        );

        let viewport = RendererState::create_viewport(swapchain.as_ref().unwrap());

        RendererState {
            backend,
            device,
            nes_screen_buffer,
            img_desc_pool,
            uniform_desc_pool,
            vertex_buffer: Some(vertex_buffer),
            vertex_memory: Some(vertex_memory),
            uniform,
            render_pass,
            pipeline,
            swapchain,
            framebuffer,
            viewport,
        }
    }

    fn recreate_swapchain(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();

        self.swapchain.take().unwrap();

        self.swapchain =
            Some(unsafe { SwapchainState::new(&mut self.backend, Rc::clone(&self.device), DIMS) });

        self.render_pass = unsafe {
            RenderPassState::new(self.swapchain.as_ref().unwrap(), Rc::clone(&self.device))
        };

        self.framebuffer = unsafe {
            FramebufferState::new(
                Rc::clone(&self.device),
                &self.render_pass,
                self.swapchain.as_mut().unwrap(),
                &COLOR_RANGE,
            )
        };

        self.pipeline = unsafe {
            PipelineState::new(
                vec![self.nes_screen_buffer.get_layout(), self.uniform.layout()],
                self.render_pass.render_pass.as_ref().unwrap(),
                Rc::clone(&self.device),
            )
        };

        self.viewport = RendererState::create_viewport(self.swapchain.as_ref().unwrap());
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

    pub fn render_frame(&mut self, recreate_swapchain: bool) -> RenderStatus {
        let mut render_status = if recreate_swapchain {
            self.recreate_swapchain();
            RenderStatus::NormalAndSwapchainRecreated
        } else {
            RenderStatus::Normal
        };

        let semaphore_index = self.framebuffer.next_acq_pre_pair_index();

        let frame: SwapImageIndex = unsafe {
            let (acquire_semaphore, _) = self.framebuffer.get_frame_data(None, semaphore_index).1;
            match self
                .swapchain
                .as_mut()
                .expect("Swapchain shouldn't be None")
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
                .borrow()
                .device
                .wait_for_fence(framebuffer_fence, !0)
                .unwrap();
            self.device
                .borrow()
                .device
                .reset_fence(framebuffer_fence)
                .unwrap();
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
                    self.nes_screen_buffer.descriptor_set(),
                    self.uniform.descriptor_set(),
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

            self.device.borrow_mut().queues.queues[0].submit(submission, Some(framebuffer_fence));

            // present frame
            if self
                .swapchain
                .as_ref()
                .unwrap()
                .swapchain
                .as_ref()
                .unwrap()
                .present(
                    &mut self.device.borrow_mut().queues.queues[0],
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
        let device = self.device.borrow();
        device.device.wait_idle().unwrap();
        unsafe {
            device.device.destroy_descriptor_pool(
                self.img_desc_pool
                    .take()
                    .expect("Image descriptor pool shouldn't be None"),
            );

            device.device.destroy_descriptor_pool(
                self.uniform_desc_pool
                    .take()
                    .expect("Uniform descriptor pool shouldn't be None"),
            );

            device.device.destroy_buffer(
                self.vertex_buffer
                    .take()
                    .expect("Vertex buffer shouldn't be None"),
            );

            device.device.free_memory(
                self.vertex_memory
                    .take()
                    .expect("Vertex memory shouldn't be None"),
            );

            self.swapchain.take();
        }
    }
}
