use crate::app_result::{AppError, AppResult};
use std::collections::HashSet;
use std::iter;
use std::sync::{Arc, OnceLock};
use tracing::{debug, enabled, error, info, trace, warn, Level};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::QueueFlags;
use vulkano::instance::debug::{
    DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger,
    DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions};
use vulkano::swapchain::Surface;
use vulkano::{Version, VulkanLibrary};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct HelloTriangleApplication {
    physical_device: Arc<PhysicalDevice>,
    _debug_utils_messenger: Option<DebugUtilsMessenger>,
    instance: Arc<Instance>,
    window: Arc<Window>,
    event_loop: EventLoop<()>,
}

impl HelloTriangleApplication {
    pub fn new(validate: bool) -> AppResult<Self> {
        let (event_loop, window) = init_window()?;
        let (instance, debug_utils_messenger, physical_device) =
            init_vulkan(&event_loop, validate)?;

        Ok(Self {
            physical_device,
            _debug_utils_messenger: debug_utils_messenger,
            instance,
            window: Arc::new(window),
            event_loop,
        })
    }

    pub fn run(self) {
        self.main_loop();
    }

    fn main_loop(self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {}
                _ => {}
            }
        });
    }
}

#[derive(Default)]
struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    _ne: NonExhaustive,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

fn find_queue_families(physical_device: &Arc<PhysicalDevice>) -> QueueFamilyIndices {
    let mut queue_family_indices = QueueFamilyIndices::default();
    for (i, prop) in physical_device.queue_family_properties().iter().enumerate() {
        if prop.queue_flags.contains(QueueFlags::GRAPHICS) {
            queue_family_indices = QueueFamilyIndices {
                graphics_family: Some(i as u32),
                ..queue_family_indices
            };
            if queue_family_indices.is_complete() {
                break;
            }
        }
    }
    queue_family_indices
}

fn pick_physical_device(instance: Arc<Instance>) -> AppResult<Arc<PhysicalDevice>> {
    instance
        .enumerate_physical_devices()?
        .find(is_device_suitable)
        .ok_or(AppError::PhysicalDevices)
}

fn is_device_suitable(physical_device: &Arc<PhysicalDevice>) -> bool {
    let queue_family_indices = find_queue_families(physical_device);

    queue_family_indices.is_complete()
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

fn validation_layers() -> &'static HashSet<String> {
    static VALIDATION_LAYERS_LOCK: OnceLock<HashSet<String>> = OnceLock::new();
    VALIDATION_LAYERS_LOCK.get_or_init(|| VALIDATION_LAYERS.iter().map(|s| s.to_string()).collect())
}

fn init_window() -> AppResult<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    window.set_inner_size(PhysicalSize::new(WIDTH, HEIGHT));
    window.set_title("Vulkan Tutorial");

    Ok((event_loop, window))
}

fn init_vulkan(
    event_loop: &EventLoop<()>,
    validate: bool,
) -> AppResult<(
    Arc<Instance>,
    Option<DebugUtilsMessenger>,
    Arc<PhysicalDevice>,
)> {
    let instance = create_instance(event_loop, validate)?;
    let debug_utils_messenger = if validate {
        Some(setup_debug_messenger(instance.clone())?)
    } else {
        None
    };
    let physical_device = pick_physical_device(instance.clone())?;

    Ok((instance, debug_utils_messenger, physical_device))
}

fn create_instance(event_loop: &EventLoop<()>, validate: bool) -> AppResult<Arc<Instance>> {
    let library = VulkanLibrary::new()?;

    let required_extensions = InstanceExtensions {
        ext_debug_utils: validate,
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

    if validate {
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

fn populate_debug_utils_messenger_create_info() -> DebugUtilsMessengerCreateInfo {
    DebugUtilsMessengerCreateInfo {
        message_severity: DebugUtilsMessageSeverity::ERROR
            | DebugUtilsMessageSeverity::WARNING
            | DebugUtilsMessageSeverity::INFO
            | DebugUtilsMessageSeverity::VERBOSE,
        message_type: DebugUtilsMessageType::GENERAL
            | DebugUtilsMessageType::VALIDATION
            | DebugUtilsMessageType::PERFORMANCE,
        ..DebugUtilsMessengerCreateInfo::user_callback(debug_utils_messenger_callback())
    }
}

fn debug_utils_messenger_callback() -> Arc<DebugUtilsMessengerCallback> {
    // SAFETY: func does not make any calls to the Vulkan API
    unsafe {
        DebugUtilsMessengerCallback::new(|message_severity, message_type, callback_data| {
            if message_severity.intersects(DebugUtilsMessageSeverity::ERROR) {
                error!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else if message_severity.intersects(DebugUtilsMessageSeverity::WARNING) {
                warn!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else if message_severity.intersects(DebugUtilsMessageSeverity::INFO) {
                info!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else if message_severity.intersects(DebugUtilsMessageSeverity::VERBOSE) {
                debug!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            } else {
                trace!(
                    "[{message_type:?}] validation layer: {}",
                    callback_data.message
                );
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct NonExhaustive;