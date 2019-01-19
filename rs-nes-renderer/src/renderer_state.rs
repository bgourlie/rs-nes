use std::{iter, mem::size_of};

use gfx_hal::{
    buffer, command, memory, pool::CommandPoolCreateFlags, Backend, Device, Graphics, MemoryType,
    QueueGroup, Surface,
};

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

        let (mut device, mut queues) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        let nes_screen = NesScreen::new(
            &mut device,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            limits,
            &memory_types,
        );

        let (vertex_memory, vertex_buffer, palette_memory, palette_buffer) = unsafe {
            let (
                vertex_staging_memory,
                vertex_staging_buffer,
                vertex_device_memory,
                vertex_device_buffer,
                vertex_memory_requirements,
            ) = Self::create_vertex_buffers_and_memory(&device, &memory_types);

            let (
                palette_staging_memory,
                palette_staging_buffer,
                palette_device_memory,
                palette_device_buffer,
                palette_memory_requirements,
            ) = Self::create_palette_buffers_and_memory(&device, &memory_types, &PALETTE);

            let mut command_pool = device
                .create_command_pool_typed::<Graphics>(&queues, CommandPoolCreateFlags::TRANSIENT)
                .unwrap();

            let mut commands = command_pool.acquire_command_buffer::<command::OneShot>();

            commands.begin();

            commands.copy_buffer(
                &vertex_staging_buffer,
                &vertex_device_buffer,
                &[command::BufferCopy {
                    src: 0,
                    dst: 0,
                    size: vertex_memory_requirements.size,
                }],
            );

            commands.copy_buffer(
                &palette_staging_buffer,
                &palette_device_buffer,
                &[command::BufferCopy {
                    src: 0,
                    dst: 0,
                    size: palette_memory_requirements.size,
                }],
            );

            commands.finish();

            queues.queues[0].submit_nosemaphores(iter::once(&commands), None);

            queues.queues[0].wait_idle().unwrap();
            command_pool.free(iter::once(commands));
            device.destroy_command_pool(command_pool.into_raw());
            device.destroy_buffer(vertex_staging_buffer);
            device.free_memory(vertex_staging_memory);
            device.destroy_buffer(palette_staging_buffer);
            device.free_memory(palette_staging_memory);
            (
                vertex_device_memory,
                vertex_device_buffer,
                palette_device_memory,
                palette_device_buffer,
            )
        };

        let (palette_uniform, swapchain) = unsafe {
            let palette_uniform = PaletteUniform::new(&mut device, palette_memory, palette_buffer);

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

    unsafe fn create_palette_buffers_and_memory(
        device: &B::Device,
        memory_types: &[MemoryType],
        data: &[f32; 256],
    ) -> (
        B::Memory,
        B::Buffer,
        B::Memory,
        B::Buffer,
        memory::Requirements,
    ) {
        let uniform_upload_size = data.len() as u64 * 4;
        println!("Uniform upload size {}", uniform_upload_size);
        let mut staging_buffer = device
            .create_buffer(uniform_upload_size, buffer::Usage::TRANSFER_SRC)
            .expect("Unable to create palette uniform buffer");

        let staging_memory_requirements = device.get_buffer_requirements(&staging_buffer);

        let staging_memory_type = memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                staging_memory_requirements.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(memory::Properties::CPU_VISIBLE)
            })
            .expect("Palette uniform memory type not supported")
            .into();

        let staging_memory = device
            .allocate_memory(staging_memory_type, staging_memory_requirements.size)
            .unwrap();
        device
            .bind_buffer_memory(&staging_memory, 0, &mut staging_buffer)
            .unwrap();

        let mut data_target = device
            .acquire_mapping_writer(&staging_memory, 0..staging_memory_requirements.size)
            .expect("Unable to acquire mapping writer");

        data_target[0..data.len()].copy_from_slice(data);

        device
            .release_mapping_writer(data_target)
            .expect("Unable to release mapping writer");

        let mut device_buffer = device
            .create_buffer(
                uniform_upload_size,
                buffer::Usage::UNIFORM | buffer::Usage::TRANSFER_DST,
            )
            .unwrap();

        let device_memory_requirements = device.get_buffer_requirements(&device_buffer);

        let device_memory_type = &memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                device_memory_requirements.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(memory::Properties::DEVICE_LOCAL)
            })
            .expect("Vertex device local memory type not supported")
            .into();

        let device_memory = device
            .allocate_memory(*device_memory_type, device_memory_requirements.size)
            .unwrap();

        device
            .bind_buffer_memory(&device_memory, 0, &mut device_buffer)
            .unwrap();

        (
            staging_memory,
            staging_buffer,
            device_memory,
            device_buffer,
            staging_memory_requirements,
        )
    }

    unsafe fn create_vertex_buffers_and_memory(
        device: &B::Device,
        memory_types: &[MemoryType],
    ) -> (
        B::Memory,
        B::Buffer,
        B::Memory,
        B::Buffer,
        memory::Requirements,
    ) {
        let vertex_stride = size_of::<Vertex>() as u64;
        let vertex_upload_size = QUAD.len() as u64 * vertex_stride;

        let mut staging_buffer = device
            .create_buffer(vertex_upload_size, buffer::Usage::TRANSFER_SRC)
            .unwrap();

        let staging_memory_requirements = device.get_buffer_requirements(&staging_buffer);

        let staging_memory_type = &memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                staging_memory_requirements.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(memory::Properties::CPU_VISIBLE)
            })
            .expect("Vertex staging buffer memory type not supported")
            .into();

        let staging_memory = device
            .allocate_memory(*staging_memory_type, staging_memory_requirements.size)
            .unwrap();

        device
            .bind_buffer_memory(&staging_memory, 0, &mut staging_buffer)
            .unwrap();

        let mut data_target = device
            .acquire_mapping_writer::<Vertex>(&staging_memory, 0..staging_memory_requirements.size)
            .expect("Unable to acquire mapping writer");

        data_target[0..QUAD.len()].copy_from_slice(&QUAD);

        device
            .release_mapping_writer(data_target)
            .expect("Unable to release mapping writer");

        let mut device_buffer = device
            .create_buffer(
                vertex_upload_size,
                buffer::Usage::VERTEX | buffer::Usage::TRANSFER_DST,
            )
            .unwrap();

        let device_memory_requirements = device.get_buffer_requirements(&device_buffer);

        let device_memory_type = &memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                device_memory_requirements.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(memory::Properties::DEVICE_LOCAL)
            })
            .expect("Vertex device local memory type not supported")
            .into();

        let device_memory = device
            .allocate_memory(*device_memory_type, device_memory_requirements.size)
            .unwrap();

        device
            .bind_buffer_memory(&device_memory, 0, &mut device_buffer)
            .unwrap();

        (
            staging_memory,
            staging_buffer,
            device_memory,
            device_buffer,
            staging_memory_requirements,
        )
    }
}

const fn is_gl_backend() -> bool {
    cfg!(feature = "gl")
}
