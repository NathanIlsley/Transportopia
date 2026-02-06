use winit::keyboard::KeyCode;

use crate::instance;
use crate::texture;
use crate::transform;

pub trait Sprite {
    fn transform(&self) -> &transform::Transform;
    fn transform_mut(&mut self) -> &mut transform::Transform;
    fn texture(&self) -> texture::Texture;
    fn interested_keys(&self) -> Vec<KeyCode>;

    fn get_instance(&self) -> instance::Instance {
        instance::Instance::new(self.transform())
    }

    fn has_same_texture<T: Sprite>(&self, other: &T) -> bool {
        self.texture() == other.texture()
    }

    #[allow(unused_variables)]
    fn key_event(&mut self, key: KeyCode, pressed: bool) {}
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
}