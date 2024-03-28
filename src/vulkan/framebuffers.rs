use anyhow::Result;
use std::sync::Arc;
use vulkano::image::view::ImageView;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};

pub fn create_framebuffers(
    render_pass: &Arc<RenderPass>,
    image_views: &[Arc<ImageView>],
) -> Result<Vec<Arc<Framebuffer>>> {
    Ok(image_views
        .iter()
        .map(|image_view| {
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![image_view.clone()],
                    ..FramebufferCreateInfo::default()
                },
            )
        })
        .collect::<Result<_, _>>()?)
}
