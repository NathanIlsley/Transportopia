use cgmath::InnerSpace;
use rendering::texture;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use transportopia::tile;
use transportopia::main_camera;

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
    let track_dimensions = 0.04 * 1.265 * cgmath::Vector2::<f32>::new(240.0, 240.0).normalize();
    let grass_dimensions = 0.04 * 1.0 * cgmath::Vector2::<f32>::new(240.0, 120.0).normalize();
    for j in -100..101 {
        for i in -100..101 {
            sprites.push(tile::Tile::new(
                grass_dimensions,
                cgmath::Vector2::new(grass_dimensions.x / 2.0 * (i + j) as f32, grass_dimensions.y / 2.0 * (i - j) as f32),
                grass_texture
            ));
        }
    }
    for j in -100..101 {
        for i in -100..101 {
            sprites.push(tile::Tile::new(
                track_dimensions,
                cgmath::Vector2::new(grass_dimensions.x / 2.0 * (i + j) as f32, grass_dimensions.y / 2.0 * (i - j) as f32),
                track_texture
            ));
        }
    }
    // sprites.push(tile::Tile::new(
    //     grass_dimensions,
    //     cgmath::Vector2::new(0.0, 0.0),
    //     grass_texture
    // ));
    // sprites.push(tile::Tile::new(
    //     track_dimensions,
    //     cgmath::Vector2::new(0.0, 0.0),
    //     track_texture
    // ));
    // sprites.push(tile::Tile::new(
    //     grass_dimensions,
    //     cgmath::Vector2::new(grass_dimensions.x / 2.0, -grass_dimensions.y / 2.0),
    //     grass_texture
    // ));
    // sprites.push(tile::Tile::new(
    //     track_dimensions,
    //     cgmath::Vector2::new(grass_dimensions.x / 2.0, -grass_dimensions.y / 2.0),
    //     track_texture
    // ));
    
    let camera = main_camera::MainCamera::new(cgmath::Vector2::new(0.0, 0.0));

    rendering::run(camera, sprites, key_handler)
        .expect("Application encountered an error and had to close.");
}