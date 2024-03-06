pub struct HelloTriangleApplication;

impl HelloTriangleApplication {
    pub fn run(&self) {
        self.init_vulkan();
        self.main_loop();
    }

    fn init_vulkan(&self) {}
    fn main_loop(&self) {}
}
