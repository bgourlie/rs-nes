use gfx_hal::{
    format::Swizzle, image as i, pool, Backbuffer, Backend, CommandPool, Device, Graphics,
};

use crate::{
    device_state::DeviceState, render_pass_state::RenderPassState, swapchain_state::SwapchainState,
};

pub struct FramebufferState<B: Backend> {
    framebuffers: Option<Vec<B::Framebuffer>>,
    framebuffer_fences: Option<Vec<B::Fence>>,
    command_pools: Option<Vec<CommandPool<B, Graphics>>>,
    frame_images: Option<Vec<(B::Image, B::ImageView)>>,
    acquire_semaphores: Option<Vec<B::Semaphore>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    last_ref: usize,
}

impl<B: Backend> FramebufferState<B> {
    pub unsafe fn new(
        device: &DeviceState<B>,
        render_pass: &RenderPassState<B>,
        swapchain: &mut SwapchainState<B>,
        color_range: &i::SubresourceRange,
    ) -> Self {
        let (frame_images, framebuffers) = match swapchain.backbuffer.take().unwrap() {
            Backbuffer::Images(images) => {
                let extent = i::Extent {
                    width: swapchain.extent.width as _,
                    height: swapchain.extent.height as _,
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
                                swapchain.format,
                                Swizzle::NO,
                                color_range.clone(),
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
                            .create_framebuffer(
                                render_pass.render_pass.as_ref().unwrap(),
                                Some(rtv),
                                extent,
                            )
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
        let mut fences: Vec<B::Fence> = vec![];
        let mut command_pools: Vec<CommandPool<B, Graphics>> = vec![];
        let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        for _ in 0..iter_count {
            fences.push(device.device.create_fence(true).unwrap());
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

        FramebufferState {
            frame_images: Some(frame_images),
            framebuffers: Some(framebuffers),
            framebuffer_fences: Some(fences),
            command_pools: Some(command_pools),
            present_semaphores: Some(present_semaphores),
            acquire_semaphores: Some(acquire_semaphores),
            last_ref: 0,
        }
    }

    pub fn next_acq_pre_pair_index(&mut self) -> usize {
        if self.last_ref >= self.acquire_semaphores.as_ref().unwrap().len() {
            self.last_ref = 0
        }

        let ret = self.last_ref;
        self.last_ref += 1;
        ret
    }

    #[allow(clippy::type_complexity)]
    pub fn get_frame_data(
        &mut self,
        frame_id: Option<usize>,
        semaphore_index: usize,
    ) -> (
        Option<(
            &mut B::Fence,
            &mut B::Framebuffer,
            &mut CommandPool<B, Graphics>,
        )>,
        (&mut B::Semaphore, &mut B::Semaphore),
    ) {
        (
            if let Some(frame_id) = frame_id {
                Some((
                    &mut self.framebuffer_fences.as_mut().unwrap()[frame_id],
                    &mut self.framebuffers.as_mut().unwrap()[frame_id],
                    &mut self.command_pools.as_mut().unwrap()[frame_id],
                ))
            } else {
                None
            },
            (
                &mut self.acquire_semaphores.as_mut().unwrap()[semaphore_index],
                &mut self.present_semaphores.as_mut().unwrap()[semaphore_index],
            ),
        )
    }

    pub fn destroy_resources(state: &mut Self, device: &B::Device) {
        unsafe {
            for fence in state
                .framebuffer_fences
                .take()
                .expect("Fences shouldn't be None")
            {
                device.wait_for_fence(&fence, !0).unwrap();
                device.destroy_fence(fence);
            }

            for command_pool in state
                .command_pools
                .take()
                .expect("Command pools shouldn't be None")
            {
                device.destroy_command_pool(command_pool.into_raw());
            }

            for acquire_semaphore in state
                .acquire_semaphores
                .take()
                .expect("Acquire semaphores shouldn't be None")
            {
                device.destroy_semaphore(acquire_semaphore);
            }

            for present_semaphore in state
                .present_semaphores
                .take()
                .expect("Present semaphores shouldn't be None")
            {
                device.destroy_semaphore(present_semaphore);
            }

            for framebuffer in state
                .framebuffers
                .take()
                .expect("Framebuffers shouldn't be None")
            {
                device.destroy_framebuffer(framebuffer);
            }

            for (_, rtv) in state
                .frame_images
                .take()
                .expect("Frame images shouldn't be None")
            {
                device.destroy_image_view(rtv);
            }
        }
    }
}
