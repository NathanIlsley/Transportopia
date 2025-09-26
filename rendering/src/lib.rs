mod instance;
mod vertex;
mod texture;
mod state;
mod app;
pub mod sprite;
pub mod camera;

use winit::event_loop::{EventLoop, ActiveEventLoop};
use winit::keyboard::KeyCode;

pub fn run<C: camera::Camera + Clone + 'static, S: sprite::Sprite + Clone + 'static>(camera: C, sprites: Vec<S>, textures: Vec<&'static [u8]>, key_handler: fn(&ActiveEventLoop, KeyCode, bool)) -> anyhow::Result<()> {
    // Using env_logger to improve error messages from wgpu
    env_logger::init();
    // Create event loop for winit
    let event_loop = EventLoop::with_user_event().build()?;
    // Create App object
    let mut app = app::App::new(camera, sprites, textures, key_handler);
    // Run the application
    event_loop.run_app(&mut app)?;

    Ok(())
}