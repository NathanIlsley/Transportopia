use macroquad::prelude::*;
use crate::world_drawer::WorldDrawer;

pub struct InputHandler {
    scroll_speed: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            scroll_speed: 200.0,
        }
    }

    pub fn take_input(&self, world_drawer: &mut WorldDrawer) {
        if is_key_down(KeyCode::Left) {
            world_drawer.change_scroll(get_frame_time() * self.scroll_speed * vec2(1.0, 0.0));
        }
        if is_key_down(KeyCode::Right) {
            world_drawer.change_scroll(get_frame_time() * self.scroll_speed * vec2(-1.0, 0.0));
        }
        if is_key_down(KeyCode::Up) {
            world_drawer.change_scroll(get_frame_time() * self.scroll_speed * vec2(0.0, 1.0));
        }
        if is_key_down(KeyCode::Down) {
            world_drawer.change_scroll(get_frame_time() * self.scroll_speed * vec2(0.0, -1.0));
        }
        if is_key_pressed(KeyCode::Equal) {
            world_drawer.change_zoom(1);
        }
        if is_key_pressed(KeyCode::Minus) {
            world_drawer.change_zoom(-1);
        }
    }
}