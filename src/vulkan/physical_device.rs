use crate::app_result::{AppError, AppResult};
use crate::vulkan::swapchain::SwapChainSupportDetails;
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
) -> AppResult<(
    Arc<PhysicalDevice>,
    QueueFamilyIndices,
    SwapChainSupportDetails,
)> {
    instance
        .enumerate_physical_devices()?
        .filter(|physical_device| {
            physical_device
                .supported_extensions()
                .contains(&DEVICE_EXTENSIONS)
        })
        .find_map(
            |physical_device| match QueueFamilyIndices::find(&physical_device, surface) {
                Ok(Some(queue_family_indices)) => {
                    match SwapChainSupportDetails::query(&physical_device, surface) {
                        Ok(swap_chain_support) => swap_chain_support
                            .is_adequate()
                            .then_some(queue_family_indices)
                            .map(|queue_family_indices| {
                                (physical_device, queue_family_indices, swap_chain_support)
                            }),
                        Err(e) => {
                            warn!(
                                "can not query swapchain support \
                                 for physical device [{physical_device:?}], \
                                 with err: {e}, skipping it"
                            );
                            None
                        }
                    }
                }
                Ok(None) => None,
                Err(e) => {
                    warn!(
                        "can not find queue families for device [{physical_device:?}], \
                         with last err: {e}, skipping it"
                    );
                    None
                }
            },
        )
        .ok_or(AppError::PhysicalDevices)
}

pub const DEVICE_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};
