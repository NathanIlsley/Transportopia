use winit::keyboard::KeyCode;
use cgmath;

use crate::instance;
use crate::texture;
use crate::transform;

pub trait Sprite {
    fn transform(&self) -> &transform::Transform;
    fn transform_mut(&mut self) -> &mut transform::Transform;
    fn texture(&self) -> &texture::Texture;
    fn interested_keys(&self) -> Vec<KeyCode>;

    fn get_instance(&self) -> instance::Instance {
        let scale_matrix = cgmath::Matrix4::from_nonuniform_scale(self.transform().dimensions().x, self.transform().dimensions().y, 1.0);
        instance::Instance::new((cgmath::Matrix4::from_translation(self.transform().position().extend(0.0)) * scale_matrix).into())
    }

    // fn change_handled(&mut self);

    #[allow(unused_variables)]
    fn key_event(&mut self, key: KeyCode, pressed: bool) {}
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
}