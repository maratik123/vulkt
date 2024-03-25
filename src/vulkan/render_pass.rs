use anyhow::Result;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::image::ImageLayout;
use vulkano::render_pass::{
    AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, RenderPass,
    RenderPassCreateInfo, SubpassDescription,
};

pub fn create_render_pass(device: &Arc<Device>) -> Result<Arc<RenderPass>> {
    let color_attachment = AttachmentDescription {
        load_op: AttachmentLoadOp::Clear,
        store_op: AttachmentStoreOp::Store,
        final_layout: ImageLayout::PresentSrc,
        ..AttachmentDescription::default()
    };
    let attachment_reference = AttachmentReference {
        attachment: 0,
        layout: ImageLayout::ColorAttachmentOptimal,
        ..AttachmentReference::default()
    };
    let subpass = SubpassDescription {
        color_attachments: vec![Some(attachment_reference)],
        ..SubpassDescription::default()
    };
    Ok(RenderPass::new(
        device.clone(),
        RenderPassCreateInfo {
            attachments: vec![color_attachment],
            subpasses: vec![subpass],
            ..RenderPassCreateInfo::default()
        },
    )?)
}
