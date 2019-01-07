use gfx_hal::{image as i, pass, pso::PipelineStage, Backend, Device};

use crate::swapchain_state::SwapchainState;

pub struct RenderPassState<B: Backend> {
    pub render_pass: Option<B::RenderPass>,
}

impl<B: Backend> RenderPassState<B> {
    pub unsafe fn new(swapchain: &SwapchainState<B>, device: &B::Device) -> Self {
        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(swapchain.format),
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
                .create_render_pass(&[attachment], &[subpass], &[dependency])
                .ok()
        };

        RenderPassState { render_pass }
    }

    pub fn destroy_resources(state: &mut Self, device: &B::Device) {
        unsafe {
            device.destroy_render_pass(
                state
                    .render_pass
                    .take()
                    .expect("Renderpass shouldn't be None"),
            );
        }
    }
}
