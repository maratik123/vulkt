use crate::app_result::{AppError, AppResult};
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};
use tracing::{debug, error, info, trace, warn};
use vulkano::instance::debug::{
    DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger,
    DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use vulkano::{Version, VulkanLibrary};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn validation_layers() -> &'static HashSet<String> {
    static VALIDATION_LAYERS: OnceLock<HashSet<String>> = OnceLock::new();
    VALIDATION_LAYERS.get_or_init(|| {
        ["VK_LAYER_KHRONOS_validation"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    })
}

pub struct HelloTriangleApplication {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
    instance: Arc<Instance>,
    _debug_utils_messenger: Option<DebugUtilsMessenger>,
}

impl HelloTriangleApplication {
    pub fn new(validate: bool) -> AppResult<Self> {
        let (event_loop, window) = init_window()?;
        let instance = init_vulkan(&event_loop, validate)?;
        let _debug_utils_messenger = if validate {
            Some(setup_debug_messenger(instance.clone())?)
        } else {
            None
        };
        let app = Self {
            event_loop,
            window: Arc::new(window),
            instance,
            _debug_utils_messenger,
        };
        Ok(app)
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

fn init_vulkan(event_loop: &EventLoop<()>, validate: bool) -> AppResult<Arc<Instance>> {
    create_instance(event_loop, validate)
}

fn create_instance(event_loop: &EventLoop<()>, validate: bool) -> AppResult<Arc<Instance>> {
    let library = VulkanLibrary::new()?;

    let supported_extensions = library.supported_extensions();
    debug!("available extensions: {supported_extensions:?}");

    let mut required_extensions = Surface::required_extensions(&event_loop);
    if validate {
        required_extensions.ext_debug_utils = true;
    }
    info!("required extensions: {required_extensions:?}");

    let unavailable_required_extensions = required_extensions - *supported_extensions;
    info!("unavailable required extensions: {unavailable_required_extensions:?}");

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

        if required_layers
            .difference(&available_layers)
            .next()
            .is_some()
        {
            error!(
                "unavailable required layers: {:?}",
                required_layers.difference(&available_layers)
            );
            return Err(AppError::RequiredLayers);
        }

        instance_create_info.enabled_layers = required_layers.iter().cloned().collect();

        instance_create_info.debug_utils_messengers =
            vec![populate_debug_utils_messenger_create_info()];
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
