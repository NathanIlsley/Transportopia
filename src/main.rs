use rendering::run;
use rendering::sprite::Sprite;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

#[derive(Clone)]
struct Tile {
    width: f32,
    height: f32,
    position: (f32, f32),
}

impl Tile {
    fn new(width: f32, height: f32, position: (f32, f32)) -> Self {
        Self { width, height, position }
    }
}

impl Sprite for Tile {
    fn width(&self) -> f32 {self.width}
    fn height(&self) -> f32 {self.height}
    fn position(&self) -> (f32, f32) {self.position}

    fn start(&mut self) {
    }

    fn update(&mut self, delta_time: f64) {
        // println!("Delta time: {}", delta_time);
    }
}

fn key_handler(event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
    match (key, pressed) {
        (KeyCode::Escape, true) => event_loop.exit(),
        _ => {}
    }
}

fn main() {
    let grass_texture: &'static [u8] = include_bytes!("..\\assets\\grass_0.png");
    let mut sprites = vec![];
    for j in -500..501 {
        for i in -500..501 {
            sprites.push(Tile::new(0.05, 0.05, (0.025 * (i + j) as f32, 0.025 * (i - j) as f32)));
        }
    }

    run(sprites, vec![grass_texture], key_handler)
        .unwrap_or_else(|_| {println!("Application encountered an error and had to close.")});
}