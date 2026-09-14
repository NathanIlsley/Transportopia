use macroquad::prelude::*;
use crate::rendering::Rendering;
use crate::rendering::Rendering;

pub struct InputHandler {
    scroll_speed: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            scroll_speed: 200.0,
        }
    }

    pub fn take_input(&self, rendering: &mut Rendering) {
        if is_key_down(KeyCode::Left) {
            rendering.change_scroll(get_frame_time() * self.scroll_speed * vec2(1.0, 0.0));
        }
        if is_key_down(KeyCode::Right) {
            rendering.change_scroll(get_frame_time() * self.scroll_speed * vec2(-1.0, 0.0));
        }
        if is_key_down(KeyCode::Up) {
            rendering.change_scroll(get_frame_time() * self.scroll_speed * vec2(0.0, 1.0));
        }
        if is_key_down(KeyCode::Down) {
            rendering.change_scroll(get_frame_time() * self.scroll_speed * vec2(0.0, -1.0));
        }
        if is_key_pressed(KeyCode::Equal) {
            rendering.change_zoom(0.2);
        }
        if is_key_pressed(KeyCode::Minus) {
            rendering.change_zoom(-0.2);
        }
    }
}