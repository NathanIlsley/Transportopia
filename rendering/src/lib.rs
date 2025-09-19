mod instance;
mod vertex;
mod texture;
mod state;
mod app;

use winit::event_loop::EventLoop;

pub fn run() -> anyhow::Result<()> {
    // Using env_logger to improve error messages from wgpu
    env_logger::init();

    // Create event loop for winit
    let event_loop = EventLoop::with_user_event().build()?;
    // Create App object
    let mut app = app::App::new();
    // Run the application
    event_loop.run_app(&mut app)?;

    Ok(())
}