mod instance;
mod vertex;
mod state;
mod app;
mod system;
pub mod texture;
pub mod sprite;
pub mod camera;
pub mod transform;

use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::keyboard::KeyCode;

pub fn run<C: camera::Camera + 'static, S: sprite::Sprite + PartialEq + 'static>(camera: C, sprites: Vec<S>, key_handler: fn(&ActiveEventLoop, KeyCode, bool)) -> anyhow::Result<()> {
    // Using env_logger to improve error messages from wgpu
    env_logger::init();
    // Create event loop for winit
    let event_loop = EventLoop::with_user_event().build()?;
    // Create App object
    let mut app = app::App::new(camera, sprites, key_handler);
    // Run the application
    event_loop.run_app(&mut app)?;

    Ok(())
}