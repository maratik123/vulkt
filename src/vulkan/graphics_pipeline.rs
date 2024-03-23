use crate::app_error::AppError;
use crate::vulkan::shader::{load_fragment, load_vertex};
use anyhow::Result;
use smallvec::smallvec;
use std::collections::HashSet;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::{CullMode, FrontFace, RasterizationState};
use vulkano::pipeline::graphics::vertex_input::VertexInputState;
use vulkano::pipeline::graphics::viewport::{Scissor, Viewport, ViewportState};
use vulkano::pipeline::layout::{PipelineDescriptorSetLayoutCreateInfo, PipelineLayoutCreateInfo};
use vulkano::pipeline::{DynamicState, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::swapchain::Swapchain;
use winit::dpi::PhysicalSize;

pub fn create_graphics_pipeline(
    device: &Arc<Device>,
    swapchain: &Arc<Swapchain>,
) -> Result<Arc<PipelineLayout>> {
    let vert_shader_module = load_vertex(device.clone())?;
    let frag_shader_module = load_fragment(device.clone())?;

    let vert_shader_entry_point = vert_shader_module
        .entry_point("main")
        .ok_or(AppError::EntryPointNotFound)?;

    let frag_shader_entry_point = frag_shader_module
        .entry_point("main")
        .ok_or(AppError::EntryPointNotFound)?;

    let shader_stages = [
        PipelineShaderStageCreateInfo::new(vert_shader_entry_point),
        PipelineShaderStageCreateInfo::new(frag_shader_entry_point),
    ];

    let _pipeline_layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&shader_stages)
            .into_pipeline_layout_create_info(device.clone())?,
    );

    let dynamic_states = HashSet::from([DynamicState::Viewport, DynamicState::Scissor]);

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

    let color_blend_attachment = ColorBlendAttachmentState {
        ..ColorBlendAttachmentState::default()
    };

    let color_blending = ColorBlendState::with_attachment_states(1, color_blend_attachment);

    let pipeline_layout_info = PipelineLayoutCreateInfo::default();

    let pipeline_layout = PipelineLayout::new(device.clone(), pipeline_layout_info)?;

    Ok(pipeline_layout)
}
