use std::mem::size_of;

use gfx_hal::{
    buffer, memory,
    pso::{self, ShaderStageFlags},
    Backend, Device, Graphics, Limits, MemoryType,
};

use crate::{
    backend_resources::BackendResources, descriptor_set::DescSetLayout, device_state::DeviceState,
    nes_screen::NesScreen, palette::PALETTE, palette_uniform::PaletteUniform,
    swapchain_state::SwapchainState, vertex::Vertex, DIMS, QUAD,
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
    pub memory_types: Vec<MemoryType>,
    pub limits: Limits,
    pub window: Option<Window>,
    uniform_desc_pool: B::DescriptorPool,
    img_desc_pool: B::DescriptorPool,
    swapchain: Option<SwapchainState<B>>,
    device: DeviceState<B>,
    vertex_memory: B::Memory,
    vertex_buffer: B::Buffer,
    palette_uniform: PaletteUniform<B>,
    viewport: pso::Viewport,
    nes_screen_buffer: NesScreen<B>,
}

impl<B: Backend> RendererState<B> {
    pub fn new(backend_resources: BackendResources<B>) -> Self {
        let (mut surface, adapter, limits, memory_types, window) = backend_resources.take();

        if !is_gl_backend() && window.is_none() {
            panic!("Window shouldn't be None")
        }

        let mut device = DeviceState::new(adapter, &surface);

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
                .expect("Unable to create image descriptor pool")
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
                .expect("Unable to create uniform descriptor pool")
        };

        let (image_desc, uniform_desc) = unsafe {
            (
                image_desc.create_desc_set(&mut img_desc_pool),
                uniform_desc.create_desc_set(&mut uniform_desc_pool),
            )
        };

        let nes_screen_buffer = NesScreen::new::<Graphics>(
            &mut device,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            image_desc,
            limits,
            &memory_types,
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

            let memory_type = &memory_types
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

        let (palette_uniform, swapchain) = unsafe {
            let palette_uniform =
                PaletteUniform::new(&mut device.device, &memory_types, &PALETTE, uniform_desc);

            let swapchain_state = SwapchainState::new(
                &mut surface,
                &device,
                vec![nes_screen_buffer.layout(), palette_uniform.layout()],
                DIMS,
            );
            (palette_uniform, swapchain_state)
        };

        let viewport = RendererState::create_viewport(&swapchain);

        RendererState {
            limits,
            memory_types,
            surface,
            window,
            device,
            nes_screen_buffer,
            img_desc_pool,
            uniform_desc_pool,
            vertex_buffer,
            vertex_memory,
            palette_uniform,
            viewport,
            swapchain: Some(swapchain),
        }
    }

    fn recreate_swapchain(&mut self) {
        self.device.device.wait_idle().unwrap();

        unsafe {
            let old_swapchain = self.swapchain.take().unwrap();
            old_swapchain.destroy(&self.device.device);
            self.swapchain = Some(SwapchainState::new(
                &mut self.surface,
                &self.device,
                vec![
                    self.nes_screen_buffer.layout(),
                    self.palette_uniform.layout(),
                ],
                DIMS,
            ));
        }

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

        self.nes_screen_buffer
            .update_buffer_data(screen_buffer, &self.device.device);

        // The following line causing huge memory leak with dx12 backend
        // See https://github.com/gfx-rs/gfx/issues/2556
        // TODO: Refactor so that the buffer copy reuses command buffer instead of creating its own
        self.nes_screen_buffer
            .copy_buffer_to_texture(&mut self.device);

        self.nes_screen_buffer
            .wait_for_transfer_completion(&self.device.device);

        let acquire_semaphore_index = self.swapchain.as_mut().unwrap().next_acq_pre_pair_index();

        let next_image_index = {
            let image_index = self
                .swapchain
                .as_mut()
                .unwrap()
                .next_swap_image_index(acquire_semaphore_index);
            if image_index.is_none() {
                return RenderStatus::RecreateSwapchain;
            }
            image_index.unwrap()
        };

        self.swapchain
            .as_mut()
            .unwrap()
            .wait_for_image_fence(next_image_index, &self.device.device);
        if !self.swapchain.as_mut().unwrap().present(
            next_image_index,
            &self.viewport,
            &mut self.device.queues,
            &self.vertex_buffer,
            self.nes_screen_buffer.descriptor_set(),
            self.palette_uniform.descriptor_set(),
        ) {
            render_status = RenderStatus::RecreateSwapchain
        }

        render_status
    }

    pub fn destroy(mut self) {
        self.device.device.wait_idle().unwrap();
        unsafe {
            self.device
                .device
                .destroy_descriptor_pool(self.img_desc_pool);

            self.device
                .device
                .destroy_descriptor_pool(self.uniform_desc_pool);

            self.device.device.destroy_buffer(self.vertex_buffer);
            self.device.device.free_memory(self.vertex_memory);
        }

        self.palette_uniform.destroy(&self.device.device);
        self.nes_screen_buffer.destroy(&self.device.device);
        self.swapchain.take().unwrap().destroy(&self.device.device);
    }
}

const fn is_gl_backend() -> bool {
    cfg!(feature = "gl")
}
