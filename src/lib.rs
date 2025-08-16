use macroquad::prelude::*;

pub mod structures;

pub fn draw_grass(scroll: &Vec2, tile_dim: &Vec2, scale: &f32, texture: &Texture2D) {
    for i in -15..15 {
        for j in -15..15 {
            draw_texture_ex(
                &texture,
                screen_width() / 2.0 + (i - 1 - j) as f32 * tile_dim.x * scale + scroll.x,
                screen_height() / 2.0 + (j - 1 + i) as f32 * tile_dim.y * scale + scroll.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(texture.width() * scale, texture.height() * scale)),
                    ..Default::default()
                },
            );
        }
    }
}