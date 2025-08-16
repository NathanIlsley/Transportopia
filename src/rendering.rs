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
        let scale = 0.01;
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
        set_camera(&self.rt_camera);

        for i in -0..1 {
            for j in -0..1 {
                draw_texture_ex(
                    &self.grass,
                    self.render_target.texture.width() / 2.0 + (i - 1 - j) as f32 * self.tile_dim.x,
                    self.render_target.texture.height() / 2.0 + (j - 1 + i) as f32 * self.tile_dim.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(self.grass.width(), self.grass.height())),
                        ..Default::default()
                    },
                );
            }
        }
        
        set_default_camera();

        for i in -15..16 {
            for j in -15..16 {
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

        // for i in -15..15 {
        //     for j in -15..15 {
        //         draw_texture_ex(
        //             &texture,
        //             screen_width() / 2.0 + (i - 1 - j) as f32 * tile_dim.x * scale + scroll.x,
        //             screen_height() / 2.0 + (j - 1 + i) as f32 * tile_dim.y * scale + scroll.y,
        //             WHITE,
        //             DrawTextureParams {
        //                 dest_size: Some(Vec2::new(texture.width() * scale, texture.height() * scale)),
        //                 ..Default::default()
        //             },
        //         );
        //     }
        // }

        for structure in &self.structures {
            structure.draw(vec2(0.0, 0.0), &self.scale, &self.tile_dim, &self.scroll);
        }
    }
}