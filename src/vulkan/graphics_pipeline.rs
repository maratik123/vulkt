use crate::app_result::AppResult;
use crate::vulkan::shader::{load_fragment, load_vertex};
use std::sync::Arc;
use vulkano::device::Device;

pub fn create_graphics_pipeline(device: &Arc<Device>) -> AppResult<()> {
    let vert_shader_module = load_vertex(device.clone())?;
    let frag_shader_module = load_fragment(device.clone())?;

    Ok(())
}
