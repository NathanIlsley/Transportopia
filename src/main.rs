use macroquad::prelude::*;

#[macroquad::main("Transportopia")]
async fn main() {
    let scale = 0.25;

    let grass: Texture2D = load_texture("./assets/grass_0.png").await.unwrap();
    grass.set_filter(FilterMode::Nearest);

    // x and y dimensions of the base game tile measured from the centre of the tile to each corner
    let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);
    
    let s_track_0: Texture2D = load_texture("./assets/track_straight_0.png").await.unwrap();
    s_track_0.set_filter(FilterMode::Nearest);
    let s_track_0_offset = vec2(0.0, -60.0);
    
    let c_track_0: Texture2D = load_texture("./assets/track_curved_0.png").await.unwrap();
    c_track_0.set_filter(FilterMode::Nearest);
    let c_track_0_offset = vec2(120.0,30.0);
    
    
    loop {
        clear_background(BLACK);
        
        draw_grass(&vec2(0.0, 0.0), &tile_dim, &scale, &grass);
        
        draw_texture_ex(
            &s_track_0,
            screen_width() / 2.0 - tile_dim.x * scale + s_track_0_offset.x * scale,
            screen_height() / 2.0 - tile_dim.y * scale + s_track_0_offset.y * scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(s_track_0.width() * scale, s_track_0.height() * scale)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &s_track_0,
            screen_width() / 2.0 + 0.0 * tile_dim.x * scale + s_track_0_offset.x * scale,
            screen_height() / 2.0 + 0.0 * tile_dim.y * scale + s_track_0_offset.y * scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(s_track_0.width() * scale, s_track_0.height() * scale)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &c_track_0,
            screen_width() / 2.0 - 0.0 * tile_dim.x * scale + c_track_0_offset.x * scale,
            screen_height() / 2.0 - 0.0 * tile_dim.y * scale + c_track_0_offset.y * scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(c_track_0.width() * scale, c_track_0.height() * scale)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

fn draw_grass(scroll: &Vec2, tile_dim: &Vec2, scale: &f32, grass: &Texture2D) {
    for i in -20..20 {
        for j in -20..20 {
            draw_texture_ex(
                &grass,
                screen_width() / 2.0 + (i - 1 - j) as f32 * tile_dim.x * scale + scroll.x,
                screen_height() / 2.0 + (j - 1 + i) as f32 * tile_dim.y * scale + scroll.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(grass.width() * scale, grass.height() * scale)),
                    ..Default::default()
                },
            );
        }
    }
}