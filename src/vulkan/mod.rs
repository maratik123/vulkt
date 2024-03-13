mod debug;

use crate::app_result::{AppError, AppResult, QueueFamilyType};
use crate::vulkan::debug::populate_debug_utils_messenger_create_info;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::iter;
use std::sync::{Arc, OnceLock};
use tracing::{debug, enabled, error, info, warn, Level};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::format::Format;
use vulkano::instance::debug::DebugUtilsMessenger;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions};
use vulkano::swapchain::{PresentMode, Surface, SurfaceCapabilities, SurfaceInfo};
use vulkano::{Version, VulkanLibrary};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct AppVulkan {
    pub present_queue: Arc<Queue>,
    pub graphics_queue: Arc<Queue>,
    pub device: Arc<Device>,
    pub physical_device: Arc<PhysicalDevice>,
    pub surface: Arc<Surface>,
    pub debug_utils_messenger: Option<DebugUtilsMessenger>,
    pub instance: Arc<Instance>,
}

impl AppVulkan {
    pub fn init(
        event_loop: &EventLoop<()>,
        window: Arc<Window>,
        enable_validation: bool,
    ) -> AppResult<Self> {
        let instance = create_instance(event_loop, enable_validation)?;
        let debug_utils_messenger = if enable_validation {
            Some(setup_debug_messenger(instance.clone())?)
        } else {
            None
        };
        let surface = create_surface(instance.clone(), window)?;
        let (physical_device, queue_family_indices) = pick_physical_device(&instance, &surface)?;
        let CreateLogicalDeviceResult {
            device,
            graphics_queue,
            present_queue,
            ..
        } = create_logical_device(physical_device.clone(), queue_family_indices)?;

        Ok(Self {
            instance,
            debug_utils_messenger,
            surface,
            physical_device,
            device,
            graphics_queue,
            present_queue,
        })
    }
}

fn create_instance(
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
        let available_layers = library
            .layer_properties()
            .unwrap()
            .map(|layer| layer.name().to_string())
            .collect();
        debug!("available layers: {available_layers:?}");

        let required_layers = validation_layers();
        info!("required layers: {required_layers:?}");

        let mut diff_it = required_layers.difference(&available_layers);
        if let Some(first_diff) = diff_it.next() {
            error!(
                "unavailable required layers: {:?}",
                iter::once(first_diff).chain(diff_it).collect::<Vec<_>>()
            );
            return Err(AppError::RequiredLayers);
        } else {
            info!("all required layers satisfied");
        }

        instance_create_info = InstanceCreateInfo {
            enabled_layers: required_layers.iter().cloned().collect(),
            debug_utils_messengers: vec![populate_debug_utils_messenger_create_info()],
            ..instance_create_info
        };
    }

    Ok(Instance::new(library, instance_create_info)?)
}

fn setup_debug_messenger(instance: Arc<Instance>) -> AppResult<DebugUtilsMessenger> {
    Ok(DebugUtilsMessenger::new(
        instance,
        populate_debug_utils_messenger_create_info(),
    )?)
}

fn create_surface(instance: Arc<Instance>, window: Arc<Window>) -> AppResult<Arc<Surface>> {
    Ok(Surface::from_window(instance, window)?)
}

const DEVICE_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};

fn pick_physical_device(
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

struct CreateLogicalDeviceResult {
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
}

fn create_logical_device(
    physical_device: Arc<PhysicalDevice>,
    queue_family_indices: QueueFamilyIndices,
) -> AppResult<CreateLogicalDeviceResult> {
    let queue_create_infos = HashSet::from([
        queue_family_indices.graphics_family,
        queue_family_indices.present_family,
    ])
    .into_iter()
    .map(|queue_family_index| QueueCreateInfo {
        queue_family_index,
        queues: vec![1.0],
        ..QueueCreateInfo::default()
    })
    .collect();
    let device_features = Features::default();
    let device_create_info = DeviceCreateInfo {
        queue_create_infos,
        enabled_features: device_features,
        enabled_extensions: DEVICE_EXTENSIONS,
        ..DeviceCreateInfo::default()
    };
    let (device, queues) = Device::new(physical_device, device_create_info)?;
    let queues: SmallVec<[_; 2]> = queues.collect();

    fn find_queue<'a>(
        queues: impl IntoIterator<Item = &'a Arc<Queue>>,
        queue_family_index: u32,
        queue_type: QueueFamilyType,
    ) -> AppResult<Arc<Queue>> {
        Ok(queues
            .into_iter()
            .find(|queue| queue.queue_family_index() == queue_family_index)
            .ok_or_else(|| AppError::QueueForDevice(queue_type))?
            .clone())
    }

    Ok(CreateLogicalDeviceResult {
        device,
        graphics_queue: find_queue(
            &queues,
            queue_family_indices.graphics_family,
            QueueFamilyType::Graphics,
        )?,
        present_queue: find_queue(
            &queues,
            queue_family_indices.present_family,
            QueueFamilyType::Present,
        )?,
    })
}

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

fn validation_layers() -> &'static HashSet<String> {
    static VALIDATION_LAYERS_LOCK: OnceLock<HashSet<String>> = OnceLock::new();
    VALIDATION_LAYERS_LOCK.get_or_init(|| VALIDATION_LAYERS.iter().map(|s| s.to_string()).collect())
}

#[derive(Default)]
struct QueueFamilyIndicesBuilder {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct QueueFamilyIndices {
    graphics_family: u32,
    present_family: u32,
}

impl QueueFamilyIndicesBuilder {
    fn build(&self) -> Option<QueueFamilyIndices> {
        Some(QueueFamilyIndices {
            graphics_family: self.graphics_family?,
            present_family: self.present_family?,
        })
    }
}

struct SwapChainSupportDetails {
    _capabilities: SurfaceCapabilities,
    formats: Vec<Format>,
    present_mode: Vec<PresentMode>,
}

impl SwapChainSupportDetails {
    fn is_adequate(&self) -> bool {
        !self.formats.is_empty() && !self.present_mode.is_empty()
    }
}

fn query_swap_chain_support(
    physical_device: &PhysicalDevice,
    surface: &Surface,
) -> AppResult<SwapChainSupportDetails> {
    let capabilities = physical_device.surface_capabilities(surface, SurfaceInfo::default())?;
    let formats = physical_device
        .surface_formats(surface, SurfaceInfo::default())?
        .into_iter()
        .map(|(format, _)| format)
        .collect();
    let present_mode = physical_device
        .surface_present_modes(surface, SurfaceInfo::default())?
        .collect();
    Ok(SwapChainSupportDetails {
        _capabilities: capabilities,
        formats,
        present_mode,
    })
}

fn find_queue_families(
    physical_device: &PhysicalDevice,
    surface: &Surface,
) -> AppResult<Option<QueueFamilyIndices>> {
    let mut queue_family_indices = QueueFamilyIndicesBuilder::default();
    for (i, prop) in physical_device.queue_family_properties().iter().enumerate() {
        let i = i as u32;
        let mut changed = false;

        if queue_family_indices.graphics_family.is_none()
            && prop.queue_flags.contains(QueueFlags::GRAPHICS)
        {
            changed |= true;
            queue_family_indices.graphics_family = Some(i);
        }

        if queue_family_indices.present_family.is_none()
            && physical_device.surface_support(i, surface)?
        {
            changed |= true;
            queue_family_indices.present_family = Some(i);
        }

        if changed {
            if let queue_family_indices @ Some(_) = queue_family_indices.build() {
                return Ok(queue_family_indices);
            }
        }
    }
    Ok(None)
}
