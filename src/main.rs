use macroquad::prelude::*;

#[macroquad::main("Display PNG")]
async fn main() {
    let scale = 0.25;

    let grass: Texture2D = load_texture("./assets/grass_0.png").await.unwrap();
    grass.set_filter(FilterMode::Nearest);


    loop {
        clear_background(BLACK);

        draw_grass(Vec2 { x: 0.0, y: 0.0 }, &grass, &scale);

        next_frame().await;
    }
}

fn draw_grass(offset: Vec2, grass: &Texture2D, scale: &f32) {
    for i in 0..5 {
        for j in 0..5 {
            draw_texture_ex(
                &grass,
                screen_width() / 2.0 - grass.width() * scale / 2.0 + i as f32 * grass.width() * scale + offset.x,
                screen_height() / 2.0 - grass.height() * scale / 2.0 + j as f32 * grass.height() * scale + offset.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(grass.width() * scale, grass.height() * scale)),
                    ..Default::default()
                },
            );
        }
    }
}