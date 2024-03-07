use crate::app_result::AppResult;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct HelloTriangleApplication {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

impl HelloTriangleApplication {
    pub fn new() -> AppResult<HelloTriangleApplication> {
        let (event_loop, window) = HelloTriangleApplication::init_window()?;
        Ok(HelloTriangleApplication {
            event_loop,
            window: Arc::new(window),
        })
    }

    pub fn run(self) {
        self.init_vulkan();
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

    fn init_vulkan(&self) {}

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
