use vulkt::hello_triangle_application::HelloTriangleApplication;

fn main() {
    tracing_subscriber::fmt::init();

    let app = HelloTriangleApplication::new().unwrap();

    app.run();
}
