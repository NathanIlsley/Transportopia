use rendering::{transform, texture, sprite};
use winit::keyboard::KeyCode;

pub struct Tile {
    transform: transform::Transform,
    texture: texture::Texture,
}

impl Tile {
    pub fn new(dimensions: cgmath::Vector2<f32>, position: cgmath::Vector2<f32>, texture: texture::Texture) -> Self {
        Self { 
            transform: rendering::transform::Transform::new(dimensions, position),
            texture,
        }
    }
}

impl sprite::Sprite for Tile {
    fn transform(&self) -> &rendering::transform::Transform {&self.transform}
    fn transform_mut(&mut self) -> &mut rendering::transform::Transform {&mut self.transform}
    fn texture(&self) -> texture::Texture {self.texture}

    fn interested_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::Space]
    }

    fn key_event(&mut self, key: KeyCode, pressed: bool) {
        match (key, pressed) {
            (KeyCode::Space, true) => {
                self.transform.set_visible(!self.transform.visible());
            }
            _ => {}
        }
    }

    // fn start(&mut self) {

    // }

    // fn update(&mut self, delta_time: f64) {

    // }
}