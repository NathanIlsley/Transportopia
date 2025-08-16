use macroquad::prelude::*;

pub struct Structure {
    pub offset: Vec2,
    pub shape: Vec2,
    pub texture: Texture2D,
}

impl Structure {
    pub async fn new(offset: Vec2, shape: Vec2, texture: &str) -> Self {
        let texture = load_texture(texture).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            offset,
            shape,
            texture,
        }
    }

    pub fn draw(&self, position: Vec2, scale: &f32, tiledim: &Vec2, scroll: &Vec2) {
        draw_texture_ex(
            &self.texture,
            screen_width() / 2.0 + (position.x - 1.0 - position.y) * tiledim.x * scale + self.offset.x * scale + scroll.x,
            screen_height() / 2.0 + (position.y - 1.0 + position.x) * tiledim.y * scale + self.offset.y * scale + scroll.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.texture.width() * scale, self.texture.height() * scale)),
                ..Default::default()
            },
        );
    }
}