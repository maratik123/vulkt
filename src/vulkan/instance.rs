use crate::app_result::{AppError, AppResult};
use crate::vulkan::debug::populate_debug_utils_messenger_create_info;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::iter;
use std::sync::{Arc, OnceLock};
use tracing::{debug, enabled, error, info, Level};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions};
use vulkano::swapchain::Surface;
use vulkano::{Version, VulkanLibrary};
use winit::event_loop::EventLoop;

pub fn create_instance(
    event_loop: &EventLoop<()>,
    enable_validation: bool,
) -> AppResult<Arc<Instance>> {
    let library = VulkanLibrary::new()?;

    let required_extensions = InstanceExtensions {
        ext_debug_utils: enable_validation,
        ..Surface::required_extensions(&event_loop)
    };
    info!("required extensions: {required_extensions:?}");

    if enabled!(Level::INFO) {
        let supported_extensions = library.supported_extensions();
        debug!("available extensions: {supported_extensions:?}");

        info!(
            "unavailable required extensions: {:?}",
            required_extensions - *supported_extensions
        );
    }

    let mut instance_create_info = InstanceCreateInfo {
        engine_name: Some("No Engine".to_string()),
        engine_version: Version::V1_0,
        enabled_extensions: required_extensions,
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..InstanceCreateInfo::application_from_cargo_toml()
    };

    if enable_validation {
        let required_layers = validation_layers();
        info!("required layers: {required_layers:?}");

        {
            let layer_properties: Vec<_> = library.layer_properties()?.collect();
            let available_layers = layer_properties.iter().map(|layer| layer.name()).collect();
            debug!("available layers: {available_layers:?}");

            let mut diff_it = required_layers.difference(&available_layers);
            if let Some(first_diff) = diff_it.next() {
                error!(
                    "unavailable required layers: {:?}",
                    iter::once(first_diff)
                        .chain(diff_it)
                        .collect::<SmallVec<[_; VALIDATION_LAYERS.len()]>>()
                );
                return Err(AppError::RequiredLayers);
            } else {
                info!("all required layers satisfied");
            }
        }

        instance_create_info = InstanceCreateInfo {
            enabled_layers: required_layers.iter().map(|s| s.to_string()).collect(),
            debug_utils_messengers: vec![populate_debug_utils_messenger_create_info()],
            ..instance_create_info
        };
    }

    Ok(Instance::new(library, instance_create_info)?)
}

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

#[inline]
fn validation_layers() -> &'static HashSet<&'static str> {
    static VALIDATION_LAYERS_LOCK: OnceLock<HashSet<&'static str>> = OnceLock::new();
    VALIDATION_LAYERS_LOCK.get_or_init(|| VALIDATION_LAYERS.iter().copied().collect())
}
