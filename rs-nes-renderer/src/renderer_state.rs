use std::mem::size_of;

use gfx_hal::{buffer, memory, Backend, Device, Graphics, QueueGroup, Surface};

use crate::{
    backend_resources::BackendResources, nes_screen::NesScreen, palette::PALETTE,
    palette_uniform::PaletteUniform, swapchain_state::SwapchainState, vertex::Vertex, DIMS, QUAD,
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
    _window: Option<Window>,
    swapchain: Option<SwapchainState<B>>,
    vertex_memory: B::Memory,
    vertex_buffer: B::Buffer,
    palette_uniform: PaletteUniform<B>,
    nes_screen: NesScreen<B>,
    device: B::Device,
    physical_device: B::PhysicalDevice,
    queues: QueueGroup<B, Graphics>,
}

impl<B: Backend> RendererState<B> {
    pub fn new(backend_resources: BackendResources<B>) -> Self {
        let (mut surface, adapter, limits, memory_types, window) = backend_resources.take();

        if !is_gl_backend() && window.is_none() {
            panic!("Window shouldn't be None")
        }

        let (mut device, queues) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        let nes_screen = NesScreen::new(
            &mut device,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            limits,
            &memory_types,
        );

        let (vertex_memory, vertex_buffer) = {
            let vertex_stride = size_of::<Vertex>() as u64;
            let vertex_upload_size = QUAD.len() as u64 * vertex_stride;

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
            let palette_uniform = PaletteUniform::new(&mut device, &memory_types, &PALETTE);

            let swapchain_state = SwapchainState::new(
                &mut surface,
                &device,
                &adapter.physical_device,
                &queues,
                DIMS,
                &nes_screen,
                &palette_uniform,
                &vertex_buffer,
            );
            (palette_uniform, swapchain_state)
        };

        RendererState {
            surface,
            _window: window,
            device,
            nes_screen,
            vertex_buffer,
            vertex_memory,
            palette_uniform,
            physical_device: adapter.physical_device,
            queues,
            swapchain: Some(swapchain),
        }
    }

    fn recreate_swapchain(&mut self) {
        self.device.wait_idle().unwrap();

        unsafe {
            let old_swapchain = self.swapchain.take().unwrap();
            old_swapchain.destroy(&self.device);
            self.swapchain = Some(SwapchainState::new(
                &mut self.surface,
                &self.device,
                &self.physical_device,
                &self.queues,
                DIMS,
                &self.nes_screen,
                &self.palette_uniform,
                &self.vertex_buffer,
            ));
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
            .update_buffer_data(screen_buffer, &self.device);

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
            .wait_for_image_fence(next_image_index, &self.device);

        if !self
            .swapchain
            .as_mut()
            .unwrap()
            .present(next_image_index, &mut self.queues)
        {
            render_status = RenderStatus::RecreateSwapchain
        }

        render_status
    }

    pub fn destroy(mut self) {
        self.device.wait_idle().expect("Wait idle failed");
        unsafe {
            self.device.destroy_buffer(self.vertex_buffer);
            self.device.free_memory(self.vertex_memory);
        }
        self.palette_uniform.destroy(&self.device);
        self.nes_screen.destroy(&self.device);
        self.swapchain.take().unwrap().destroy(&self.device);
    }
}

const fn is_gl_backend() -> bool {
    cfg!(feature = "gl")
}
