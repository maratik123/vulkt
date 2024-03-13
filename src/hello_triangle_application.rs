use crate::app_result::AppResult;
use crate::vulkan::AppVulkan;
use crate::window::AppWindow;
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, Queue};
use vulkano::instance::debug::DebugUtilsMessenger;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct HelloTriangleApplication {
    _present_queue: Arc<Queue>,
    _graphics_queue: Arc<Queue>,
    _device: Arc<Device>,
    _physical_device: Arc<PhysicalDevice>,
    _surface: Arc<Surface>,
    _debug_utils_messenger: Option<DebugUtilsMessenger>,
    _instance: Arc<Instance>,
    window: Arc<Window>,
    event_loop: EventLoop<()>,
}

impl HelloTriangleApplication {
    pub fn new(enable_validation: bool) -> AppResult<Self> {
        let (event_loop, window) = AppWindow::init()?;
        let window = Arc::new(window);
        let AppVulkan {
            instance,
            debug_utils_messenger,
            surface,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            ..
        } = AppVulkan::init(&event_loop, window.clone(), enable_validation)?;

        Ok(Self {
            _present_queue: present_queue,
            _graphics_queue: graphics_queue,
            _device: device,
            _physical_device: physical_device,
            _surface: surface,
            _debug_utils_messenger: debug_utils_messenger,
            _instance: instance,
            window,
            event_loop,
        })
    }

    pub fn run(self) -> ! {
        self.main_loop()
    }

    fn main_loop(self) -> ! {
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
        })
    }
}
