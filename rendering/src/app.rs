use crate::{camera, sprite, state, texture};

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::*, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window
};

pub(crate) struct App<C: camera::Camera, S: sprite::Sprite> {
    state: Option<state::State<C, S>>,
    camera: Option<C>,
    sprites: Option<Vec<S>>,
    textures: Option<Vec<texture::Texture>>,
    key_handler: fn(&ActiveEventLoop, KeyCode, bool),
}

impl<C: camera::Camera, S: sprite::Sprite> App<C, S> {
    pub(crate) fn new(camera: C, sprites: Vec<S>, textures: Vec<texture::Texture>, key_handler: fn(&ActiveEventLoop, KeyCode, bool)) -> Self {        
        Self {
            state: None,
            camera: Some(camera),
            sprites: Some(sprites),
            textures: Some(textures),
            key_handler,
        }
    }
}

impl<C: camera::Camera + 'static, S: sprite::Sprite + 'static> ApplicationHandler<state::State<C, S>> for App<C, S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Use default values for window
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        // Set an Arc pointer to point to the new window
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Use pollster to wait until the State struct has been created
        self.state = Some(pollster::block_on(state::State::new(window, self.camera.take().unwrap(), self.sprites.take().unwrap(), self.textures.take().unwrap())).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: state::State<C, S>) {
        // Winit allows a new object of type <T> to be passed by EventLoopProxy::send_event which runs this
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // If self.state is None, do nothing
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        // Handle the event
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                // If a redraw is requested, update and render state and handle errors with render
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.get_window_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            // Send keyboard input event to state to be handled
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                state.handle_key(code, key_state.is_pressed());
                (self.key_handler)(event_loop, code, key_state.is_pressed());
            },
            _ => {}
        }
    }
}