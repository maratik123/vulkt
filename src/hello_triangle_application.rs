use crate::vulkan::AppVulkan;
use crate::window::AppWindow;
use anyhow::Result;
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, Queue};
use vulkano::image::view::ImageView;
use vulkano::image::Image;
use vulkano::instance::debug::DebugUtilsMessenger;
use vulkano::instance::Instance;
use vulkano::pipeline::PipelineLayout;
use vulkano::swapchain::{Surface, Swapchain};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct HelloTriangleApplication {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
    _instance: Arc<Instance>,
    _debug_utils_messenger: Option<DebugUtilsMessenger>,
    _surface: Arc<Surface>,
    _physical_device: Arc<PhysicalDevice>,
    _device: Arc<Device>,
    _graphics_queue: Arc<Queue>,
    _present_queue: Arc<Queue>,
    _swapchain: Arc<Swapchain>,
    _swapchain_images: Vec<Arc<Image>>,
    _swapchain_image_views: Vec<Arc<ImageView>>,
    _pipeline_layout: Arc<PipelineLayout>,
}

impl HelloTriangleApplication {
    pub fn new(enable_validation: bool) -> Result<Self> {
        let AppWindow { event_loop, window } = AppWindow::init()?;
        let window = Arc::new(window);
        let AppVulkan {
            instance,
            debug_utils_messenger,
            surface,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            swapchain,
            swapchain_images,
            swapchain_image_views,
            pipeline_layout,
        } = AppVulkan::init(&event_loop, &window, enable_validation)?;

        Ok(Self {
            _pipeline_layout: pipeline_layout,
            _swapchain_image_views: swapchain_image_views,
            _swapchain_images: swapchain_images,
            _swapchain: swapchain,
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

    #[inline]
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
