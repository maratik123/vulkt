use crate::app_result::{AppError, AppResult};
use crate::vulkan::queue_family_indices::find_queue_families;
use crate::vulkan::swapchain::query_swap_chain_support;
use crate::vulkan::QueueFamilyIndices;
use std::sync::Arc;
use tracing::warn;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::DeviceExtensions;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

pub fn pick_physical_device(
    instance: &Arc<Instance>,
    surface: &Surface,
) -> AppResult<(Arc<PhysicalDevice>, QueueFamilyIndices)> {
    instance
        .enumerate_physical_devices()?
        .filter(|physical_device| physical_device.supported_extensions().contains(&DEVICE_EXTENSIONS))
        .filter(|physical_device| {
            match query_swap_chain_support(physical_device, surface) {
                Ok(swap_chain_support_details) => swap_chain_support_details.is_adequate(),
                Err(e) => {
                    warn!("can not query swapchain support for physical device [{physical_device:?}], with err: {e}, skipping it");
                    false
                },
            }
        })
        .find_map(|physical_device| {
            match find_queue_families(&physical_device, surface) {
                Ok(queue_family_indices) => queue_family_indices
                    .map(|queue_family_indices| (physical_device, queue_family_indices)),
                Err(e) => {
                    warn!("can not find queue families for device [{physical_device:?}], with err: {e}, skipping it");
                    None
                }
            }
        })
        .ok_or(AppError::PhysicalDevices)
}

pub const DEVICE_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};
