use crate::app_result::AppResult;
use std::sync::Arc;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use vulkano::{Version, VulkanLibrary};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

pub struct HelloTriangleApplication {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
    instance: Arc<Instance>,
}

impl HelloTriangleApplication {
    pub fn new() -> AppResult<HelloTriangleApplication> {
        let (event_loop, window) = HelloTriangleApplication::init_window()?;
        let instance = HelloTriangleApplication::init_vulkan(&event_loop)?;
        let app = HelloTriangleApplication {
            event_loop,
            window: Arc::new(window),
            instance,
        };
        Ok(app)
    }

    pub fn run(self) {
        self.main_loop();
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

    fn init_vulkan(event_loop: &EventLoop<()>) -> AppResult<Arc<Instance>> {
        HelloTriangleApplication::create_instance(event_loop)
    }

    fn create_instance(event_loop: &EventLoop<()>) -> AppResult<Arc<Instance>> {
        let library = VulkanLibrary::new()?;
        let required_extensions = Surface::required_extensions(&event_loop);
        Ok(Instance::new(
            library,
            InstanceCreateInfo {
                engine_name: Some("No Engine".to_string()),
                engine_version: Version::V1_0,
                enabled_extensions: required_extensions,
                ..InstanceCreateInfo::application_from_cargo_toml()
            },
        )?)
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
