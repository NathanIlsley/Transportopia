use rendering::{sprite, camera, texture, transform};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

struct MainCamera {
    buffer: Option<camera::CameraBuffer>,
    position: cgmath::Vector2<f32>,

    delta_times: Vec<f64>,
    movement: cgmath::Vector2<f32>,
}

impl MainCamera {
    fn new(position: cgmath::Vector2<f32>) -> Self {
        Self {
            buffer: None,
            position,
            delta_times: vec![],
            movement: cgmath::Vector2::new(0.0, 0.0),
        }
    }
}

impl camera::Camera for MainCamera {
    fn buffer(&self) -> &Option<camera::CameraBuffer> {&self.buffer}
    fn position(&self) -> cgmath::Vector2<f32> {self.position}
    fn interested_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp, KeyCode::ArrowDown]
    }

    fn init_buffer(&mut self, buffer: camera::CameraBuffer) {
        self.buffer = Some(buffer);
    }

    fn key_event(&mut self, key: KeyCode, pressed: bool) {
        if key == KeyCode::ArrowLeft{
            if pressed {
                self.movement.x += 0.5;
            } else {
                self.movement.x -= 0.5;
            }
        }
        if key == KeyCode::ArrowRight{
            if pressed {
                self.movement.x += -0.5;
            } else {
                self.movement.x -= -0.5;
            }
        }
        if key == KeyCode::ArrowUp{
            if pressed {
                self.movement.y += -0.5;
            } else {
                self.movement.y -= -0.5;
            }
        }
        if key == KeyCode::ArrowDown{
            if pressed {
                self.movement.y += 0.5;
            } else {
                self.movement.y -= 0.5;
            }
        }
    }

    fn start(&mut self) {
        
    }

    fn update(&mut self, delta_time: f64) {
        // self.delta_times.push(delta_time);
        // if self.delta_times.len() > 10 {
        //     println!("fps: {}", self.delta_times.len() as f64 / self.delta_times.iter().sum::<f64>());
        //     self.delta_times = vec![];
        // }
        
        self.position += self.movement * delta_time as f32;
    }
}

struct Tile {
    transform: transform::Transform,
    texture: texture::Texture,
}

impl Tile {
    fn new(dimensions: cgmath::Vector2<f32>, position: cgmath::Vector2<f32>, texture: texture::Texture) -> Self {
        Self { 
            transform: rendering::transform::Transform::new(dimensions, position),
            texture,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.texture == other.texture
    }
}

impl sprite::Sprite for Tile {
    fn transform(&self) -> &rendering::transform::Transform {&self.transform}
    fn transform_mut(&mut self) -> &mut rendering::transform::Transform {&mut self.transform}
    fn texture(&self) -> &texture::Texture {&self.texture}

    fn interested_keys(&self) -> Vec<KeyCode> {
        vec![]
    }

    // fn key_event(&mut self, key: KeyCode, pressed: bool) {

    // }

    // fn start(&mut self) {

    // }

    // fn update(&mut self, delta_time: f64) {

    // }
}

fn key_handler(event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
    match (key, pressed) {
        (KeyCode::Escape, true) => event_loop.exit(),
        _ => {}
    }
}

fn main() {
    let grass_texture = texture::Texture::new(include_bytes!("..\\assets\\grass_0.png"), "grass_texture");
    let track_texture = texture::Texture::new(include_bytes!("..\\assets\\track_straight_0.png"), "track_texture");
    let mut sprites = vec![];
    for j in -5..6 {
        for i in -5..6 {
            sprites.push(Tile::new(cgmath::Vector2::new(0.2, 0.2), cgmath::Vector2::new(0.1 * (i + j) as f32, 0.1 * (i - j) as f32), grass_texture));
        }
    }
    sprites.push(Tile::new(cgmath::Vector2::new(0.2, 0.2), cgmath::Vector2::new(0.0, 0.0), track_texture));

    let camera = MainCamera::new(cgmath::Vector2::new(0.0, 0.0));

    rendering::run(camera, sprites, key_handler)
        .unwrap_or_else(|_| {println!("Application encountered an error and had to close.")});
}