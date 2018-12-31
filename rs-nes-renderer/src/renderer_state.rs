use std::{cell::RefCell, iter, rc::Rc};

use hal::{
    buffer, command, pool,
    pso::{self, PipelineStage, ShaderStageFlags},
    queue::Submission,
    Backend, Device, FrameSync, Swapchain,
};

use crate::{
    backend_state::BackendState, buffer_state::BufferState, color::Color,
    descriptor_set::DescSetLayout, device_state::DeviceState, framebuffer_state::FramebufferState,
    image_state::ImageState, pipeline_state::PipelineState, render_pass_state::RenderPassState,
    swapchain_state::SwapchainState, uniform::Uniform, vertex::Vertex, window_state::WindowState,
    SurfaceTrait, COLOR_RANGE, DIMS, QUAD,
};

pub struct RendererState<B: Backend> {
    uniform_desc_pool: Option<B::DescriptorPool>,
    img_desc_pool: Option<B::DescriptorPool>,
    swapchain: Option<SwapchainState<B>>,
    device: Rc<RefCell<DeviceState<B>>>,
    backend: BackendState<B>,
    window: WindowState,
    vertex_buffer: BufferState<B>,
    render_pass: RenderPassState<B>,
    uniform: Uniform<B>,
    pipeline: PipelineState<B>,
    framebuffer: FramebufferState<B>,
    viewport: pso::Viewport,
    image: ImageState<B>,
}

impl<B: Backend> RendererState<B> {
    pub unsafe fn new(mut backend: BackendState<B>, window: WindowState) -> Self {
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

        println!("Memory types: {:?}", backend.adapter.memory_types);

        let mut staging_pool = device
            .borrow()
            .device
            .create_command_pool_typed(
                &device.borrow().queues,
                pool::CommandPoolCreateFlags::empty(),
            )
            .expect("Can't create staging command pool");

        let mut image = ImageState::new::<hal::Graphics>(
            image_desc,
            &backend.adapter,
            buffer::Usage::TRANSFER_SRC,
            &mut device.borrow_mut(),
        );

        image.update_screen_buffer();
        image.copy_buffer_to_texture(&mut device.borrow_mut(), &mut staging_pool);

        let vertex_buffer = BufferState::new::<Vertex>(
            Rc::clone(&device),
            &QUAD,
            buffer::Usage::VERTEX,
            &backend.adapter.memory_types,
        );

        let uniform = Uniform::new(
            Rc::clone(&device),
            &backend.adapter.memory_types,
            &[1f32, 1.0f32, 1.0f32, 1.0f32],
            uniform_desc,
            0,
        );

        image.wait_for_transfer_completion();

        device
            .borrow()
            .device
            .destroy_command_pool(staging_pool.into_raw());

        let mut swapchain = Some(SwapchainState::new(&mut backend, Rc::clone(&device), DIMS));

        let render_pass = RenderPassState::new(swapchain.as_ref().unwrap(), Rc::clone(&device));

        let framebuffer = FramebufferState::new(
            Rc::clone(&device),
            &render_pass,
            swapchain.as_mut().unwrap(),
            &COLOR_RANGE,
        );

        let pipeline = PipelineState::new(
            vec![image.get_layout(), uniform.get_layout()],
            render_pass.render_pass.as_ref().unwrap(),
            Rc::clone(&device),
        );

        let viewport = RendererState::create_viewport(swapchain.as_ref().unwrap());

        RendererState {
            window,
            backend,
            device,
            image,
            img_desc_pool,
            uniform_desc_pool,
            vertex_buffer,
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
                vec![self.image.get_layout(), self.uniform.get_layout()],
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

    pub fn mainloop(&mut self)
    where
        B::Surface: SurfaceTrait,
    {
        let mut running = true;
        let mut recreate_swapchain = false;

        let mut r = 1.0f32;
        let mut g = 1.0f32;
        let mut b = 1.0f32;
        let mut a = 1.0f32;

        let mut cr = 0.8;
        let mut cg = 0.8;
        let mut cb = 0.8;

        let mut cur_color = Color::Red;
        let mut cur_value: u32 = 0;

        println!("\nInstructions:");
        println!("\tChoose whether to change the (R)ed, (G)reen or (B)lue color by pressing the appropriate key.");
        println!("\tType in the value you want to change it to, where 0 is nothing, 255 is normal and 510 is double, ect.");
        println!("\tThen press C to change the (C)lear colour or (Enter) for the image color.");
        println!(
            "\tSet {:?} color to: {} (press enter/C to confirm)",
            cur_color, cur_value
        );

        while running {
            {
                let uniform = &mut self.uniform;
                #[cfg(feature = "gl")]
                let backend = &self.backend;

                self.window.events_loop.poll_events(|event| {
                    if let winit::Event::WindowEvent { event, .. } = event {
                        #[allow(unused_variables)]
                        match event {
                            winit::WindowEvent::KeyboardInput {
                                input:
                                    winit::KeyboardInput {
                                        virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            }
                            | winit::WindowEvent::CloseRequested => running = false,
                            winit::WindowEvent::Resized(dims) => {
                                #[cfg(feature = "gl")]
                                backend.surface.get_window_t().resize(dims.to_physical(
                                    backend.surface.get_window_t().get_hidpi_factor(),
                                ));
                                recreate_swapchain = true;
                            }
                            winit::WindowEvent::KeyboardInput {
                                input:
                                    winit::KeyboardInput {
                                        virtual_keycode,
                                        state: winit::ElementState::Pressed,
                                        ..
                                    },
                                ..
                            } => {
                                if let Some(kc) = virtual_keycode {
                                    match kc {
                                        winit::VirtualKeyCode::Key0 => cur_value *= 10,
                                        winit::VirtualKeyCode::Key1 => {
                                            cur_value = cur_value * 10 + 1
                                        }
                                        winit::VirtualKeyCode::Key2 => {
                                            cur_value = cur_value * 10 + 2
                                        }
                                        winit::VirtualKeyCode::Key3 => {
                                            cur_value = cur_value * 10 + 3
                                        }
                                        winit::VirtualKeyCode::Key4 => {
                                            cur_value = cur_value * 10 + 4
                                        }
                                        winit::VirtualKeyCode::Key5 => {
                                            cur_value = cur_value * 10 + 5
                                        }
                                        winit::VirtualKeyCode::Key6 => {
                                            cur_value = cur_value * 10 + 6
                                        }
                                        winit::VirtualKeyCode::Key7 => {
                                            cur_value = cur_value * 10 + 7
                                        }
                                        winit::VirtualKeyCode::Key8 => {
                                            cur_value = cur_value * 10 + 8
                                        }
                                        winit::VirtualKeyCode::Key9 => {
                                            cur_value = cur_value * 10 + 9
                                        }
                                        winit::VirtualKeyCode::R => {
                                            cur_value = 0;
                                            cur_color = Color::Red
                                        }
                                        winit::VirtualKeyCode::G => {
                                            cur_value = 0;
                                            cur_color = Color::Green
                                        }
                                        winit::VirtualKeyCode::B => {
                                            cur_value = 0;
                                            cur_color = Color::Blue
                                        }
                                        winit::VirtualKeyCode::A => {
                                            cur_value = 0;
                                            cur_color = Color::Alpha
                                        }
                                        winit::VirtualKeyCode::Return => {
                                            match cur_color {
                                                Color::Red => r = cur_value as f32 / 255.0,
                                                Color::Green => g = cur_value as f32 / 255.0,
                                                Color::Blue => b = cur_value as f32 / 255.0,
                                                Color::Alpha => a = cur_value as f32 / 255.0,
                                            }
                                            uniform
                                                .buffer
                                                .as_mut()
                                                .unwrap()
                                                .update_data(0, &[r, g, b, a]);
                                            cur_value = 0;

                                            println!("Colour updated!");
                                        }
                                        winit::VirtualKeyCode::C => {
                                            match cur_color {
                                                Color::Red => cr = cur_value as f32 / 255.0,
                                                Color::Green => cg = cur_value as f32 / 255.0,
                                                Color::Blue => cb = cur_value as f32 / 255.0,
                                                Color::Alpha => {
                                                    error!(
                                                        "Alpha is not valid for the background."
                                                    );
                                                    return;
                                                }
                                            }
                                            cur_value = 0;

                                            println!("Background color updated!");
                                        }
                                        _ => return,
                                    }
                                    println!(
                                        "Set {:?} color to: {} (press enter/C to confirm)",
                                        cur_color, cur_value
                                    )
                                }
                            }
                            _ => (),
                        }
                    }
                });
            }

            if recreate_swapchain {
                self.recreate_swapchain();
                recreate_swapchain = false;
            }

            let sem_index = self.framebuffer.next_acq_pre_pair_index();

            let frame: hal::SwapImageIndex = unsafe {
                let (acquire_semaphore, _) = self
                    .framebuffer
                    .get_frame_data(None, Some(sem_index))
                    .1
                    .unwrap();
                match self
                    .swapchain
                    .as_mut()
                    .unwrap()
                    .swapchain
                    .as_mut()
                    .unwrap()
                    .acquire_image(!0, FrameSync::Semaphore(acquire_semaphore))
                {
                    Ok(image_index) => image_index,
                    Err(_) => {
                        recreate_swapchain = true;
                        continue;
                    }
                }
            };

            let (fid, sid) = self
                .framebuffer
                .get_frame_data(Some(frame as usize), Some(sem_index));

            let (framebuffer_fence, framebuffer, command_pool) = fid.unwrap();
            let (image_acquired, image_present) = sid.unwrap();

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
                cmd_buffer.bind_vertex_buffers(0, Some((self.vertex_buffer.get_buffer(), 0)));
                cmd_buffer.bind_graphics_descriptor_sets(
                    self.pipeline.pipeline_layout.as_ref().unwrap(),
                    0,
                    vec![
                        self.image.desc.set.as_ref().unwrap(),
                        self.uniform.desc.as_ref().unwrap().set.as_ref().unwrap(),
                    ],
                    &[],
                ); //TODO

                {
                    let mut encoder = cmd_buffer.begin_render_pass_inline(
                        self.render_pass.render_pass.as_ref().unwrap(),
                        framebuffer,
                        self.viewport.rect,
                        &[command::ClearValue::Color(command::ClearColor::Float([
                            cr, cg, cb, 1.0,
                        ]))],
                    );
                    encoder.draw(0..6, 0..1);
                }
                cmd_buffer.finish();

                let submission = Submission {
                    command_buffers: iter::once(&cmd_buffer),
                    wait_semaphores: iter::once((&*image_acquired, PipelineStage::BOTTOM_OF_PIPE)),
                    signal_semaphores: iter::once(&*image_present),
                };

                self.device.borrow_mut().queues.queues[0]
                    .submit(submission, Some(framebuffer_fence));

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
                        Some(&*image_present),
                    )
                    .is_err()
                {
                    recreate_swapchain = true;
                    continue;
                }
            }
        }
    }
}

impl<B: Backend> Drop for RendererState<B> {
    fn drop(&mut self) {
        self.device.borrow().device.wait_idle().unwrap();
        unsafe {
            self.device
                .borrow()
                .device
                .destroy_descriptor_pool(self.img_desc_pool.take().unwrap());
            self.device
                .borrow()
                .device
                .destroy_descriptor_pool(self.uniform_desc_pool.take().unwrap());
            self.swapchain.take();
        }
    }
}
