pub trait Sprite {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn position(&self) -> (f32, f32);

    fn start(&mut self) {}
    fn update(&mut self, delta_time: f64) {}
}