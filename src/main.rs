use macroquad::prelude::*;
use Transportopia::structures::Structure;

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
    let scale = 0.1;

    let grass: Texture2D = load_texture("./assets/grass_0.png").await.unwrap();
    grass.set_filter(FilterMode::Nearest);

    // x and y dimensions of the base game tile measured from the centre of the tile to each corner
    let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);
    
    // let s_track_0: Texture2D = load_texture("./assets/track_straight_0.png").await.unwrap();
    // s_track_0.set_filter(FilterMode::Nearest);
    // let s_track_0_offset = vec2(0.0, -60.0);

    // let s_track_90: Texture2D = load_texture("./assets/track_straight_90.png").await.unwrap();
    // s_track_90.set_filter(FilterMode::Nearest);
    // let s_track_90_offset = vec2(0.0, -60.0);

    // let c_track_0: Texture2D = load_texture("./assets/track_curved_0.png").await.unwrap();
    // c_track_0.set_filter(FilterMode::Nearest);
    // let c_track_0_offset = vec2(0.0, -30.0);

    
    // let c_track_45: Texture2D = load_texture("./assets/track_curved_45.png").await.unwrap();
    // c_track_45.set_filter(FilterMode::Nearest);
    // let c_track_45_offset = vec2(-1320.0, -30.0);

    // let c_track_90: Texture2D = load_texture("./assets/track_curved_90.png").await.unwrap();
    // c_track_90.set_filter(FilterMode::Nearest);
    // let c_track_90_offset = vec2(1.0, -719.0);

    // let c_track_135: Texture2D = load_texture("./assets/track_curved_135.png").await.unwrap();
    // c_track_135.set_filter(FilterMode::Nearest);
    // let c_track_135_offset = vec2(2.0, -12.0);

    // let c_track_180: Texture2D = load_texture("./assets/track_curved_180.png").await.unwrap();
    // c_track_180.set_filter(FilterMode::Nearest);
    // let c_track_180_offset = vec2(-1320.0, -330.0);

    // let c_track_225: Texture2D = load_texture("./assets/track_curved_225.png").await.unwrap();
    // c_track_225.set_filter(FilterMode::Nearest);
    // let c_track_225_offset = vec2(0.0, -330.0);

    // let c_track_270: Texture2D = load_texture("./assets/track_curved_270.png").await.unwrap();
    // c_track_270.set_filter(FilterMode::Nearest);
    // let c_track_270_offset = vec2(-479.0, -12.0);

    // let c_track_315: Texture2D = load_texture("./assets/track_curved_315.png").await.unwrap();
    // c_track_315.set_filter(FilterMode::Nearest);
    // let c_track_315_offset = vec2(-478.0, -719.0);

    let s_track_0 = Structure::new(vec2(0.0, -60.0), vec2(0.0, 0.0), "./assets/track_straight_0.png").await;
    let s_track_90 = Structure::new(vec2(0.0, -60.0), vec2(0.0, 0.0), "./assets/track_straight_90.png").await;
    let c_track_0 = Structure::new(vec2(0.0, -30.0), vec2(0.0, 0.0), "./assets/track_curved_0.png").await;
    let c_track_45 = Structure::new(vec2(-1320.0, -30.0), vec2(0.0, 0.0), "./assets/track_curved_45.png").await;
    let c_track_90 = Structure::new(vec2(1.0, -719.0), vec2(0.0, 0.0), "./assets/track_curved_90.png").await;
    let c_track_135 = Structure::new(vec2(2.0, -12.0), vec2(0.0, 0.0), "./assets/track_curved_135.png").await;
    let c_track_180 = Structure::new(vec2(-1320.0, -330.0), vec2(0.0, 0.0), "./assets/track_curved_180.png").await;
    let c_track_225 = Structure::new(vec2(0.0, -330.0), vec2(0.0, 0.0), "./assets/track_curved_225.png").await;
    let c_track_270 = Structure::new(vec2(-479.0, -12.0), vec2(0.0, 0.0), "./assets/track_curved_270.png").await;
    let c_track_315 = Structure::new(vec2(-478.0, -719.0), vec2(0.0, 0.0), "./assets/track_curved_315.png").await;

    let render_target = render_target(256, 256);
    render_target.texture.set_filter(FilterMode::Nearest);

    let rt_camera = Camera2D {
        render_target: Some(render_target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0))
    };
    
    // set_camera(&rt_camera);
    // clear_background(BLUE);
    
    // for i in -33..33 {
    //     for j in -33..33 {
    //         draw_texture_ex(
    //             &grass,
    //             render_target.texture.width() / 2.0 + (i - 1 - j) as f32 * tile_dim.x * scale,
    //             render_target.texture.height() / 2.0 + (j - 1 + i) as f32 * tile_dim.y * scale,
    //             WHITE,
    //             DrawTextureParams {
    //                 dest_size: Some(Vec2::new(grass.width() * scale, grass.height() * scale)),
    //                 ..Default::default()
    //             },
    //         );
    //     }
    // }     
    
    // set_default_camera();
    
    loop {
        clear_background(BLACK);
        
        Transportopia::draw_grass(&vec2(0.0, 0.0), &tile_dim, &scale, &grass);//&grass);

        // draw_texture_ex(
        //     &render_target.texture, 
        //     0.0,
        //     0.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(grass.width() * 67.0 * scale, grass.width() * 67.0 * scale)),
        //         ..Default::default()
        //     }
        // );

        s_track_0.draw(vec2(0.0, 0.0), &scale, &tile_dim, &vec2(0.0, 0.0));

        next_frame().await;
    }
}