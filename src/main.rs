use macroquad::prelude::*;
use transportopia::structures::Structure;
use transportopia::rendering::Renderer;

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
    let scale = 0.4;

    // let grass: Texture2D = load_texture("./assets/grass_0.png").await.unwrap();
    // grass.set_filter(FilterMode::Nearest);

    // // x and y dimensions of the base game tile measured from the centre of the tile to each corner
    // let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);

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

    let renderer = Renderer::new("./assets/grass_0.png", vec![s_track_0]).await;
    
    loop {
        clear_background(BLACK);
        
        renderer.draw();

        // s_track_0.draw(vec2(0.0, 0.0), &scale, &tile_dim, &vec2(0.0, 0.0));

        next_frame().await;
    }
}