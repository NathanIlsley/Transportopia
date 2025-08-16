use macroquad::prelude::*;
use crate::structures::Structure;

pub struct Renderer {
    pub scale: f32,
    grass: Texture2D,
    tile_dim: Vec2,
    render_target: RenderTarget,
    rt_camera: Camera2D,
    scroll: Vec2,
    structures: Vec<Structure>,
}

impl Renderer {
    pub async fn new(grass_tex: &str, structures: Vec<Structure>) -> Self {
        let scale = 1.0;
        let grass: Texture2D = load_texture(grass_tex).await.unwrap();
        grass.set_filter(FilterMode::Nearest);

        // x and y dimensions of the base game tile measured from the centre of the tile to each corner
        let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);

        let render_target = render_target(256, 256);
        render_target.texture.set_filter(FilterMode::Nearest);

        let rt_camera = Camera2D {
            render_target: Some(render_target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0))
        };

        let scroll = vec2(0.0, 0.0);

        Self {
            scale,
            tile_dim,
            grass,
            render_target,
            rt_camera,
            scroll,
            structures,
        }
    }

    pub fn draw(&self) {
        
        // Number of tiles to draw in each direction from the centre on the render target
        const TO_NEXT_TEX: i32 = 1;
        
        for i in -30..31 {
            for j in -30..31 {
                set_camera(&self.rt_camera);

                for k in -TO_NEXT_TEX..TO_NEXT_TEX+1 {
                    for l in -TO_NEXT_TEX..TO_NEXT_TEX+1 {
                        draw_texture_ex(
                            &self.grass,
                            self.render_target.texture.width() / 2.0 + (k - 1 - l) as f32 * self.tile_dim.x / (TO_NEXT_TEX * 2 + 1) as f32,
                            self.render_target.texture.height() / 2.0 + (l - 1 + k) as f32 * self.tile_dim.y / (TO_NEXT_TEX * 2 + 1) as f32,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(self.grass.width() / (TO_NEXT_TEX * 2 + 1) as f32, self.grass.height() / (TO_NEXT_TEX * 2 + 1) as f32)),
                                ..Default::default()
                            },
                        );
                    }
                }
                
                set_default_camera();

                draw_texture_ex(
                    &self.render_target.texture,
                    screen_width() / 2.0 - 1.0 * self.render_target.texture.width() / 2.0 * self.scale + (i - j) as f32 * self.tile_dim.x * self.scale + self.scroll.x,
                    screen_height() / 2.0 - 1.0 * self.render_target.texture.height() / 2.0 * self.scale + (j + i) as f32 * self.tile_dim.y * self.scale + self.scroll.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(self.render_target.texture.width() * self.scale, self.render_target.texture.height() * self.scale)),
                        ..Default::default()
                    },
                );
            }
        }

        for structure in &self.structures {
            structure.draw(vec2(0.0, 0.0), &(self.scale / (TO_NEXT_TEX * 2 + 1) as f32), &self.tile_dim, &self.scroll);
        }
    }
}