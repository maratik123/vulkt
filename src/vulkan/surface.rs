use crate::app_result::AppResult;
use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use winit::window::Window;

pub fn create_surface(instance: Arc<Instance>, window: Arc<Window>) -> AppResult<Arc<Surface>> {
    Ok(Surface::from_window(instance, window)?)
}
