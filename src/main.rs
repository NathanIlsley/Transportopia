use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Transportopia".to_owned(),
        window_width: 3143,
        window_height: 1920,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let scale = 0.002;

    let grass: Texture2D = load_texture("./assets/grass_0.png").await.unwrap();
    grass.set_filter(FilterMode::Nearest);

    // x and y dimensions of the base game tile measured from the centre of the tile to each corner
    let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);
    
    let s_track_0: Texture2D = load_texture("./assets/track_straight_0.png").await.unwrap();
    s_track_0.set_filter(FilterMode::Nearest);
    let s_track_0_offset = vec2(0.0, -60.0);

    let s_track_90: Texture2D = load_texture("./assets/track_straight_90.png").await.unwrap();
    s_track_90.set_filter(FilterMode::Nearest);
    let s_track_90_offset = vec2(0.0, -60.0);
    
    let c_track_0: Texture2D = load_texture("./assets/track_curved_0.png").await.unwrap();
    c_track_0.set_filter(FilterMode::Nearest);
    let c_track_0_offset = vec2(0.0, -30.0);

    let c_track_45: Texture2D = load_texture("./assets/track_curved_45.png").await.unwrap();
    c_track_45.set_filter(FilterMode::Nearest);
    let c_track_45_offset = vec2(-1320.0, -30.0);

    let c_track_90: Texture2D = load_texture("./assets/track_curved_90.png").await.unwrap();
    c_track_90.set_filter(FilterMode::Nearest);
    let c_track_90_offset = vec2(1.0, -719.0);

    let c_track_135: Texture2D = load_texture("./assets/track_curved_135.png").await.unwrap();
    c_track_135.set_filter(FilterMode::Nearest);
    let c_track_135_offset = vec2(2.0, -12.0);

    let c_track_180: Texture2D = load_texture("./assets/track_curved_180.png").await.unwrap();
    c_track_180.set_filter(FilterMode::Nearest);
    let c_track_180_offset = vec2(-1320.0, -330.0);

    let c_track_225: Texture2D = load_texture("./assets/track_curved_225.png").await.unwrap();
    c_track_225.set_filter(FilterMode::Nearest);
    let c_track_225_offset = vec2(0.0, -330.0);

    let c_track_270: Texture2D = load_texture("./assets/track_curved_270.png").await.unwrap();
    c_track_270.set_filter(FilterMode::Nearest);
    let c_track_270_offset = vec2(-479.0, -12.0);

    let c_track_315: Texture2D = load_texture("./assets/track_curved_315.png").await.unwrap();
    c_track_315.set_filter(FilterMode::Nearest);
    let c_track_315_offset = vec2(-478.0, -719.0);

    let render_target = render_target(2048, 2048);
    render_target.texture.set_filter(FilterMode::Nearest);

    let rt_camera = Camera2D {
        render_target: Some(render_target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 2048.0, 2048.0))
    };
    
    set_camera(&rt_camera);
    clear_background(BLUE);
    
    draw_grass(&vec2(0.0, 0.0), &tile_dim, &scale, &grass);//&grass);
    
    set_default_camera();
    
    loop {
        clear_background(BLACK);
        

        draw_texture_ex(
            &render_target.texture, 
            0.0, 
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            }
        );
        
        // draw_texture_ex(
        //     &s_track_0,
        //     screen_width() / 2.0 - 1.0 * tile_dim.x * scale + s_track_0_offset.x * scale,
        //     screen_height() / 2.0 - 1.0 * tile_dim.y * scale + s_track_0_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(s_track_0.width() * scale, s_track_0.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        

        // draw_texture_ex(
        //     &c_track_0,
        //     screen_width() / 2.0 - 0.0 * tile_dim.x * scale + c_track_0_offset.x * scale,
        //     screen_height() / 2.0 - 0.0 * tile_dim.y * scale + c_track_0_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_0.width() * scale, c_track_0.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_45,
        //     screen_width() / 2.0 - 2.0 * tile_dim.x * scale + c_track_45_offset.x * scale,
        //     screen_height() / 2.0 - 0.0 * tile_dim.y * scale + c_track_45_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_45.width() * scale, c_track_45.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_90,
        //     screen_width() / 2.0 - 0.0 * tile_dim.x * scale + c_track_90_offset.x * scale,
        //     screen_height() / 2.0 - 2.0 * tile_dim.y * scale + c_track_90_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_90.width() * scale, c_track_90.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_135,
        //     screen_width() / 2.0 - 0.0 * tile_dim.x * scale + c_track_135_offset.x * scale,
        //     screen_height() / 2.0 - 0.0 * tile_dim.y * scale + c_track_135_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_135.width() * scale, c_track_135.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_180,
        //     screen_width() / 2.0 - 2.0 * tile_dim.x * scale + c_track_180_offset.x * scale,
        //     screen_height() / 2.0 - 2.0 * tile_dim.y * scale + c_track_180_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_180.width() * scale, c_track_180.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_225,
        //     screen_width() / 2.0 - 0.0 * tile_dim.x * scale + c_track_225_offset.x * scale,
        //     screen_height() / 2.0 - 2.0 * tile_dim.y * scale + c_track_225_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_225.width() * scale, c_track_225.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_270,
        //     screen_width() / 2.0 - 2.0 * tile_dim.x * scale + c_track_270_offset.x * scale,
        //     screen_height() / 2.0 - 0.0 * tile_dim.y * scale + c_track_270_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_270.width() * scale, c_track_270.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &c_track_315,
        //     screen_width() / 2.0 - 2.0 * tile_dim.x * scale + c_track_315_offset.x * scale,
        //     screen_height() / 2.0 - 2.0 * tile_dim.y * scale + c_track_315_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(c_track_315.width() * scale, c_track_315.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        // draw_texture_ex(
        //     &s_track_90,
        //     screen_width() / 2.0 - 1.0 * tile_dim.x * scale + s_track_90_offset.x * scale,
        //     screen_height() / 2.0 - 1.0 * tile_dim.y * scale + s_track_90_offset.y * scale,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(s_track_90.width() * scale, s_track_90.height() * scale)),
        //         ..Default::default()
        //     },
        // );

        next_frame().await;
    }
}

fn draw_grass(scroll: &Vec2, tile_dim: &Vec2, scale: &f32, grass: &Texture2D) {
    for i in -1000..1000 {
        for j in -1000..1000 {
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