use std::{fs, io::Read, iter, mem};

use gfx_hal::{
    command, format as f,
    format::{AsFormat, ChannelType},
    image as i,
    pass::{self, Subpass},
    pool,
    pso::{self, PipelineStage, Viewport},
    window::Extent2D,
    Backbuffer, Backend, CommandPool, Device, FrameSync, Graphics, Primitive, QueueGroup,
    Submission, Surface, SwapImageIndex, Swapchain, SwapchainConfig,
};

use crate::{device_state::DeviceState, vertex::Vertex, FrameBufferFormat, COLOR_RANGE};

pub struct SwapchainState<B: Backend> {
    pub swapchain: B::Swapchain,
    pub extent: i::Extent,
    pub format: f::Format,
    pub render_pass: B::RenderPass,
    framebuffers: Vec<B::Framebuffer>,
    framebuffer_fences: Vec<B::Fence>,
    command_pools: Vec<CommandPool<B, Graphics>>,
    frame_images: Vec<(B::Image, B::ImageView)>,
    acquire_semaphores: Vec<B::Semaphore>,
    present_semaphores: Vec<B::Semaphore>,
    last_ref: usize,
    pub pipeline: B::GraphicsPipeline,
    pub pipeline_layout: B::PipelineLayout,
}

impl<B: Backend> SwapchainState<B> {
    pub unsafe fn new<IS>(
        surface: &mut B::Surface,
        device: &DeviceState<B>,
        desc_layouts: IS,
        dimensions: Extent2D,
    ) -> Self
    where
        IS: IntoIterator,
        IS::Item: std::borrow::Borrow<B::DescriptorSetLayout>,
    {
        let (caps, formats, _present_modes, _composite_alphas) =
            surface.compatibility(&device.physical_device);
        println!("formats: {:?}", formats);
        let format = formats.map_or(FrameBufferFormat::SELF, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .cloned()
                .unwrap_or(formats[0])
        });

        println!("Surface format: {:?}", format);
        let swap_config = SwapchainConfig::from_caps(&caps, format, dimensions);
        let extent = swap_config.extent.to_extent();
        let (swapchain, backbuffer) = device
            .device
            .create_swapchain(surface, swap_config, None)
            .expect("Can't create swapchain");

        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(format),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: i::Layout::Undefined..i::Layout::Present,
            };

            let subpass = pass::SubpassDesc {
                colors: &[(0, i::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = pass::SubpassDependency {
                passes: pass::SubpassRef::External..pass::SubpassRef::Pass(0),
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    ..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: i::Access::empty()
                    ..(i::Access::COLOR_ATTACHMENT_READ | i::Access::COLOR_ATTACHMENT_WRITE),
            };

            device
                .device
                .create_render_pass(&[attachment], &[subpass], &[dependency])
                .expect("Couldn't create render pass")
        };

        // Framebuffer stuff
        let (frame_images, framebuffers) = match backbuffer {
            Backbuffer::Images(images) => {
                let extent = i::Extent {
                    width: extent.width as _,
                    height: extent.height as _,
                    depth: 1,
                };
                let pairs = images
                    .into_iter()
                    .map(|image| {
                        let rtv = device
                            .device
                            .create_image_view(
                                &image,
                                i::ViewKind::D2,
                                format,
                                f::Swizzle::NO,
                                COLOR_RANGE.clone(),
                            )
                            .unwrap();
                        (image, rtv)
                    })
                    .collect::<Vec<_>>();
                let fbos = pairs
                    .iter()
                    .map(|&(_, ref rtv)| {
                        device
                            .device
                            .create_framebuffer(&render_pass, Some(rtv), extent)
                            .unwrap()
                    })
                    .collect();
                (pairs, fbos)
            }
            Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
        };

        let iter_count = if !frame_images.is_empty() {
            frame_images.len()
        } else {
            1 // GL can have zero
        };

        // TODO: Use SmallVec for these
        let mut framebuffer_fences: Vec<B::Fence> = vec![];
        let mut command_pools: Vec<CommandPool<B, Graphics>> = vec![];
        let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0..iter_count {
            framebuffer_fences.push(device.device.create_fence(true).unwrap());
            command_pools.push(
                device
                    .device
                    .create_command_pool_typed(
                        &device.queues,
                        pool::CommandPoolCreateFlags::empty(),
                    )
                    .expect("Can't create command pool"),
            );

            acquire_semaphores.push(device.device.create_semaphore().unwrap());
            present_semaphores.push(device.device.create_semaphore().unwrap());
        }

        let pipeline_layout = device
            .device
            .create_pipeline_layout(desc_layouts, &[(pso::ShaderStageFlags::VERTEX, 0..8)])
            .expect("Can't create pipeline layout");

        let pipeline = {
            let vs_module = {
                let glsl = fs::read_to_string("data/quad.vert").unwrap();
                let spirv: Vec<u8> =
                    glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Vertex)
                        .unwrap()
                        .bytes()
                        .map(|b| b.unwrap())
                        .collect();
                device.device.create_shader_module(&spirv).unwrap()
            };
            let fs_module = {
                let glsl = fs::read_to_string("data/quad.frag").unwrap();
                let spirv: Vec<u8> =
                    glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Fragment)
                        .unwrap()
                        .bytes()
                        .map(|b| b.unwrap())
                        .collect();
                device.device.create_shader_module(&spirv).unwrap()
            };

            const SHADER_ENTRY_NAME: &str = "main";

            let pipeline = {
                let (vs_entry, fs_entry) = (
                    pso::EntryPoint::<B> {
                        entry: SHADER_ENTRY_NAME,
                        module: &vs_module,
                        specialization: pso::Specialization::default(),
                    },
                    pso::EntryPoint::<B> {
                        entry: SHADER_ENTRY_NAME,
                        module: &fs_module,
                        specialization: pso::Specialization::default(),
                    },
                );

                let shader_entries = pso::GraphicsShaderSet {
                    vertex: vs_entry,
                    hull: None,
                    domain: None,
                    geometry: None,
                    fragment: Some(fs_entry),
                };

                let subpass = Subpass {
                    index: 0,
                    main_pass: &render_pass,
                };

                let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                    shader_entries,
                    Primitive::TriangleList,
                    pso::Rasterizer::FILL,
                    &pipeline_layout,
                    subpass,
                );
                pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                    pso::ColorMask::ALL,
                    pso::BlendState::ALPHA,
                ));
                pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                    binding: 0,
                    stride: mem::size_of::<Vertex>() as u32,
                    rate: 0,
                });

                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: f::Format::Rg32Float,
                        offset: 0,
                    },
                });
                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: pso::Element {
                        format: f::Format::Rg32Float,
                        offset: 8,
                    },
                });

                device.device.create_graphics_pipeline(&pipeline_desc, None)
            };

            device.device.destroy_shader_module(vs_module);
            device.device.destroy_shader_module(fs_module);

            pipeline.unwrap()
        };

        SwapchainState {
            swapchain,
            extent,
            format,
            acquire_semaphores,
            command_pools,
            frame_images,
            framebuffer_fences,
            framebuffers,
            present_semaphores,
            render_pass,
            pipeline,
            pipeline_layout,
            last_ref: 0,
        }
    }

    pub fn next_acq_pre_pair_index(&mut self) -> usize {
        if self.last_ref >= self.acquire_semaphores.len() {
            self.last_ref = 0
        }

        let ret = self.last_ref;
        self.last_ref += 1;
        ret
    }

    pub fn next_swap_image_index(&mut self, semaphore_index: usize) -> Option<SwapImageIndex> {
        let acquire_semaphore = &mut self.acquire_semaphores[semaphore_index];
        unsafe {
            self.swapchain
                .acquire_image(!0, FrameSync::Semaphore(acquire_semaphore))
                .ok()
        }
    }

    pub fn wait_for_image_fence(&mut self, image_index: SwapImageIndex, device: &B::Device) {
        let framebuffer_fence = &mut self.framebuffer_fences[image_index as usize];
        unsafe {
            device.wait_for_fence(framebuffer_fence, !0).unwrap();
            device.reset_fence(framebuffer_fence).unwrap();
        }
    }

    pub fn present(
        &mut self,
        image_index: SwapImageIndex,
        viewport: &Viewport,
        queues: &mut QueueGroup<B, Graphics>,
        vertex_buffer: &B::Buffer,
        nes_screen_descriptor_set: &B::DescriptorSet,
        palette_uniform_descriptor_set: &B::DescriptorSet,
    ) -> bool {
        let command_pool = &mut self.command_pools[image_index as usize];

        unsafe {
            command_pool.reset();
            let mut cmd_buffer = command_pool.acquire_command_buffer::<command::OneShot>();
            cmd_buffer.begin();
            cmd_buffer.set_viewports(0, &[viewport.clone()]);
            cmd_buffer.set_scissors(0, &[viewport.rect]);
            cmd_buffer.bind_graphics_pipeline(&self.pipeline);
            cmd_buffer.bind_vertex_buffers(0, Some((vertex_buffer, 0)));
            cmd_buffer.bind_graphics_descriptor_sets(
                &self.pipeline_layout,
                0,
                vec![nes_screen_descriptor_set, palette_uniform_descriptor_set],
                &[],
            );

            {
                let mut encoder = cmd_buffer.begin_render_pass_inline(
                    &self.render_pass,
                    &self.framebuffers[image_index as usize],
                    viewport.rect,
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
                    &self.acquire_semaphores[image_index as usize],
                    PipelineStage::BOTTOM_OF_PIPE,
                )),
                signal_semaphores: iter::once(&self.present_semaphores[image_index as usize]),
            };

            queues.queues[0].submit(
                submission,
                Some(&self.framebuffer_fences[image_index as usize]),
            );

            // present frame
            self.swapchain
                .present(
                    &mut queues.queues[0],
                    image_index,
                    Some(&self.present_semaphores[image_index as usize]),
                )
                .is_ok()
        }
    }

    pub fn destroy(self, device: &B::Device) {
        unsafe {
            device.destroy_swapchain(self.swapchain);
            device.destroy_render_pass(self.render_pass);

            for fence in self.framebuffer_fences {
                device.wait_for_fence(&fence, !0).unwrap();
                device.destroy_fence(fence);
            }

            for command_pool in self.command_pools {
                device.destroy_command_pool(command_pool.into_raw());
            }

            for acquire_semaphore in self.acquire_semaphores {
                device.destroy_semaphore(acquire_semaphore);
            }

            for present_semaphore in self.present_semaphores {
                device.destroy_semaphore(present_semaphore);
            }

            for framebuffer in self.framebuffers {
                device.destroy_framebuffer(framebuffer);
            }

            for (_, rtv) in self.frame_images {
                device.destroy_image_view(rtv);
            }

            device.destroy_graphics_pipeline(self.pipeline);
            device.destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}
