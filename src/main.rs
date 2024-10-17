mod app;

use std::env;

use app::Application;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let event_loop = EventLoop::new()?;
    let mut app = Application::new();

    Ok(event_loop.run_app(&mut app)?)
}
