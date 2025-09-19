use crate::state;

use std::sync::Arc;
use winit::{
    application::ApplicationHandler, event::*, event_loop::ActiveEventLoop, keyboard::PhysicalKey, window::Window
};

pub(crate) struct App {
    state: Option<state::State>,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler<state::State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Use default values for window
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        // Set an Arc pointer to point to the new window
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Use pollster to wait until the State struct has been created
        self.state = Some(pollster::block_on(state::State::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: state::State) {
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
                        let size = state.window.inner_size();
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
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}