use anyhow::Result;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::render_pass::RenderPass;
use vulkano::single_pass_renderpass;
use vulkano::swapchain::Swapchain;

pub fn create_render_pass(
    device: &Arc<Device>,
    swapchain: &Arc<Swapchain>,
) -> Result<Arc<RenderPass>> {
    Ok(single_pass_renderpass!(
        device.clone(),
        attachments: {
            color_attachment: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            }
        },
        pass: {
            color: [color_attachment],
            depth_stencil: {}
        }
    )?)
}
