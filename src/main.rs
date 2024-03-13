use clap::Parser;
use tracing::info;
use vulkt::hello_triangle_application::HelloTriangleApplication;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable validation layer
    #[arg(long)]
    validate: bool,
}

fn main() -> ! {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("validation status: {}", args.validate);

    let app = HelloTriangleApplication::new(args.validate).unwrap();

    app.run()
}
