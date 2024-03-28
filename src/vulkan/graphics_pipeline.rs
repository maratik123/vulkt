use crate::app_error::AppError;
use crate::vulkan::shader::{load_fragment, load_vertex};
use ahash::HashSet;
use anyhow::Result;
use smallvec::smallvec;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::{CullMode, FrontFace, RasterizationState};
use vulkano::pipeline::graphics::vertex_input::VertexInputState;
use vulkano::pipeline::graphics::viewport::{Scissor, Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineLayoutCreateInfo;
use vulkano::pipeline::{
    DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
};
use vulkano::render_pass::{RenderPass, Subpass};
use vulkano::swapchain::Swapchain;
use winit::dpi::PhysicalSize;

pub fn create_graphics_pipeline(
    device: &Arc<Device>,
    swapchain: &Arc<Swapchain>,
    render_pass: &Arc<RenderPass>,
) -> Result<(Arc<PipelineLayout>, Arc<GraphicsPipeline>)> {
    let vert_shader_module = load_vertex(device.clone())?;
    let frag_shader_module = load_fragment(device.clone())?;

    let vert_shader_entry_point = vert_shader_module
        .entry_point("main")
        .ok_or(AppError::EntryPointNotFound)?;

    let frag_shader_entry_point = frag_shader_module
        .entry_point("main")
        .ok_or(AppError::EntryPointNotFound)?;

    let shader_stages = smallvec![
        PipelineShaderStageCreateInfo::new(vert_shader_entry_point),
        PipelineShaderStageCreateInfo::new(frag_shader_entry_point),
    ];

    let dynamic_states = HashSet::from_iter([DynamicState::Viewport, DynamicState::Scissor]);

    let vertex_input_info = VertexInputState::new();

    let input_assembly = InputAssemblyState::default();

    let viewport = Viewport {
        extent: PhysicalSize::<u32>::from(swapchain.image_extent()).into(),
        ..Viewport::default()
    };

    let scissor = Scissor {
        extent: swapchain.image_extent(),
        ..Scissor::default()
    };

    let viewport_state = ViewportState {
        viewports: smallvec![viewport],
        scissors: smallvec![scissor],
        ..ViewportState::default()
    };

    let rasterizer = RasterizationState {
        cull_mode: CullMode::Back,
        front_face: FrontFace::Clockwise,
        ..RasterizationState::default()
    };

    let multisampling = MultisampleState::default();

    let color_blend_attachment = ColorBlendAttachmentState::default();

    let subpass_id = 0;

    let subpass = Subpass::from(render_pass.clone(), subpass_id)
        .ok_or(AppError::SubpassNotFound(subpass_id))?;

    let color_blending = ColorBlendState::with_attachment_states(
        subpass.num_color_attachments(),
        color_blend_attachment,
    );

    let pipeline_layout_info = PipelineLayoutCreateInfo::default();

    let pipeline_layout = PipelineLayout::new(device.clone(), pipeline_layout_info)?;

    let graphics_pipeline = GraphicsPipeline::new(
        device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: shader_stages,
            vertex_input_state: Some(vertex_input_info),
            input_assembly_state: Some(input_assembly),
            viewport_state: Some(viewport_state),
            rasterization_state: Some(rasterizer),
            multisample_state: Some(multisampling),
            color_blend_state: Some(color_blending),
            dynamic_state: dynamic_states,
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(pipeline_layout.clone())
        },
    )?;

    Ok((pipeline_layout, graphics_pipeline))
}
