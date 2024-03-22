use anyhow::Result;
use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use winit::window::Window;

#[inline]
pub fn create_surface(instance: &Arc<Instance>, window: &Arc<Window>) -> Result<Arc<Surface>> {
    Ok(Surface::from_window(instance.clone(), window.clone())?)
}
