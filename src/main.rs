use clap::Parser;
use tracing::info;
use vulkt::application::Application;

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

    let app = Application::new(args.validate).expect("Can not create app");

    app.run()
}
