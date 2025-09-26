use winit::keyboard::KeyCode;
use cgmath;

pub trait Camera {
    fn position(&self) -> cgmath::Vector2<f32>;
    fn interested_keys(&self) -> Vec<KeyCode>;

    fn get_matrix(&self) -> [[f32; 4]; 4] {
        cgmath::Matrix4::from_translation(self.position().extend(0.0)).into()
    }

    #[allow(unused_variables)]
    fn key_event(&mut self, key: KeyCode, pressed: bool) {}
    fn start(&mut self) {}
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
}