use macroquad::prelude::*;
use crate::structures::Structure;
use crate::tiles::TileManager;

pub struct WorldDrawer {
    scale: f32,
    // Number of tiles to draw in each direction from the centre on the render target
    to_text_chunk: i32,
    grass: Texture2D,
    tile_dim: Vec2,

    render_chunk: RenderTarget,
    rc_camera: Camera2D,
    all_grass_chunk: RenderTarget,
    agc_camera: Camera2D,

    scroll_vector: Vec2,
    structures: Vec<Structure>,
}

impl WorldDrawer {
    pub async fn new(grass_tex: &str, structures: Vec<Structure>) -> Self {
        let scale = 1.0;
        let grass: Texture2D = load_texture(grass_tex).await.unwrap();
        grass.set_filter(FilterMode::Nearest);

        // x and y dimensions of the base game tile measured from the centre of the tile to each corner
        let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);

        let render_chunk = render_target(256, 256);
        render_chunk.texture.set_filter(FilterMode::Nearest);

        let rc_camera = Camera2D {
            render_target: Some(render_chunk.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0))
        };

        let all_grass_chunk = render_target(256, 256);
        all_grass_chunk.texture.set_filter(FilterMode::Nearest);

        let agc_camera = Camera2D {
            render_target: Some(all_grass_chunk.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0))
        };

        let scroll = vec2(0.0, 0.0);

        Self {
            scale,
            to_text_chunk: 5,
            tile_dim,
            grass,

            render_chunk,
            rc_camera,
            all_grass_chunk,
            agc_camera,

            scroll_vector: scroll,
            structures,
        }
    }

    pub fn change_scroll(&mut self, delta: Vec2) {
        self.scroll_vector += delta;
    }

    fn assemble_render_chunk(&self) {
        set_camera(&self.rc_camera);

        for k in -self.to_text_chunk..self.to_text_chunk+1 {
            for l in -self.to_text_chunk..self.to_text_chunk+1 {
                draw_texture_ex(
                    &self.grass,
                    self.render_chunk.texture.width() / 2.0 + (k - 1 - l) as f32 * self.tile_dim.x / (self.to_text_chunk * 2 + 1) as f32,
                    self.render_chunk.texture.height() / 2.0 + (l - 1 + k) as f32 * self.tile_dim.y / (self.to_text_chunk * 2 + 1) as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(self.grass.width() / (self.to_text_chunk * 2 + 1) as f32, self.grass.height() / (self.to_text_chunk * 2 + 1) as f32)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn draw_render_chunk(&self, grid_x: i32, grid_y: i32) {
        set_default_camera();

        draw_texture_ex(
            &self.render_chunk.texture,
            screen_width() / 2.0 - 1.0 * self.render_chunk.texture.width() / 2.0 * self.scale + (grid_x - grid_y) as f32 * self.tile_dim.x * self.scale + self.scroll_vector.x,
            screen_height() / 2.0 - 1.0 * self.render_chunk.texture.height() / 2.0 * self.scale + (grid_y + grid_x) as f32 * self.tile_dim.y * self.scale + self.scroll_vector.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.render_chunk.texture.width() * self.scale, self.render_chunk.texture.height() * self.scale)),
                ..Default::default()
            },
        );
    }

    pub fn draw(&self, tile_manager: &TileManager) {
        for i in -5..6 {
            for j in -5..6 {
                self.assemble_render_chunk();

                self.draw_render_chunk(i, j);
            }
        }

        for structure in &self.structures {
            structure.draw(vec2(0.0, 0.0), &(self.scale / (self.to_text_chunk * 2 + 1) as f32), &self.tile_dim, &self.scroll_vector);
        }
    }
}