use rendering::{sprite, camera};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

#[derive(Clone)]
struct MainCamera {
    position: cgmath::Vector2<f32>,
}

impl MainCamera {
    fn new(position: cgmath::Vector2<f32>) -> Self {
        Self {
            position,
        }
    }
}

impl camera::Camera for MainCamera {
    fn position(&self) -> cgmath::Vector2<f32> {self.position}
    fn interested_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp, KeyCode::ArrowDown]
    }

    fn key_event(&mut self, key: KeyCode, pressed: bool) {
        
    }

    fn start(&mut self) {
        
    }

    fn update(&mut self, delta_time: f64) {
        println!("fps: {}", 1.0/delta_time);
        self.position.x -= 0.5 * delta_time as f32;
    }
}

#[derive(Clone)]
struct Tile {
    dimensions: cgmath::Vector2<f32>,
    position: cgmath::Vector2<f32>,
    changed: bool,

    move_left: bool,
}

impl Tile {
    fn new(dimensions: cgmath::Vector2<f32>, position: cgmath::Vector2<f32>) -> Self {
        Self { 
            dimensions,
            position,
            changed: false,

            move_left: false,
        }
    }
}

impl sprite::Sprite for Tile {
    fn dimensions(&self) -> cgmath::Vector2<f32> {self.dimensions}
    fn position(&self) -> cgmath::Vector2<f32> {self.position}
    fn changed(&self) -> bool {self.changed}
    fn interested_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp, KeyCode::ArrowDown]
    }

    fn change_handled(&mut self) {
        self.changed = false;
    }

    fn key_event(&mut self, key: KeyCode, pressed: bool) {
        // if key == KeyCode::ArrowLeft && pressed {
        //     self.move_left = true;
        //     println!("test");
        // }
        // else if key == KeyCode::ArrowLeft && !pressed {
        //     self.move_left = false;
        //     println!("stop");
        // }
    }

    fn start(&mut self) {

    }

    fn update(&mut self, delta_time: f64) {
        // if self.move_left {
            // self.position.x -= 0.5 * delta_time as f32;
            // self.changed = true;
            // println!("Moving left");
        // }
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
            sprites.push(Tile::new(cgmath::Vector2::new(0.05, 0.05), cgmath::Vector2::new(0.025 * (i + j) as f32, 0.025 * (i - j) as f32)));
        }
    }
    let camera = MainCamera::new(cgmath::Vector2::new(0.0, 0.0));

    rendering::run(camera, sprites, vec![grass_texture], key_handler)
        .unwrap_or_else(|_| {println!("Application encountered an error and had to close.")});
}