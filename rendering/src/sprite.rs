use winit::keyboard::KeyCode;
use cgmath;

use crate::instance;

pub trait Sprite {
    fn dimensions(&self) -> cgmath::Vector2<f32>;
    fn position(&self) -> cgmath::Vector2<f32>;
    fn changed(&self) -> bool;
    fn interested_keys(&self) -> Vec<KeyCode>;

    fn get_instance(&self) -> instance::Instance {
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.dimensions().x, self.dimensions().y, 1.0);
        instance::Instance::new((cgmath::Matrix4::from_translation(self.position().extend(0.0)) * scale_matrix).into())
    }

    fn change_handled(&mut self);

    #[allow(unused_variables)]
    fn key_event(&mut self, key: KeyCode, pressed: bool) {}
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
}