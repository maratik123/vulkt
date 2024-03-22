use crate::app_result::AppError;
use crate::vulkan::shader::{load_fragment, load_vertex};
use anyhow::Result;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{PipelineLayout, PipelineShaderStageCreateInfo};

pub fn create_graphics_pipeline(device: &Arc<Device>) -> Result<()> {
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

    // let create_info = GraphicsPipelineCreateInfo {
    //     ..GraphicsPipelineCreateInfo::layout()
    // };

    Ok(())
}
