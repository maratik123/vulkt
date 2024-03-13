use crate::app_result::AppResult;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

pub struct AppWindow;

impl AppWindow {
    pub fn init() -> AppResult<(EventLoop<()>, Window)> {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        window.set_inner_size(PhysicalSize::new(WIDTH, HEIGHT));
        window.set_title("Vulkan Tutorial");

        Ok((event_loop, window))
    }
}
