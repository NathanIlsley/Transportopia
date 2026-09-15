use macroquad::prelude::*;

use crate::tiles::{TerrainChunkConfig, TileManager};

pub struct Rendering {
    scale: f32,
    scroll_vector: Vec2,
    terrain: TileManager,
}

impl Rendering {
    pub async fn new() -> Self {
        Self::with_config(TerrainChunkConfig::default()).await
    }

    pub async fn with_config(config: TerrainChunkConfig) -> Self {
        let grass: Texture2D = load_texture("assets/grass_0.png").await.unwrap();
        grass.set_filter(FilterMode::Nearest);

        Self {
            scale: 2.0,
            scroll_vector: vec2(0.0, 0.0),
            terrain: TileManager::new(grass, config),
        }
    }

    pub fn change_scroll(&mut self, delta: Vec2) {
        self.scroll_vector += delta;
    }

    pub fn change_zoom(&mut self, delta: f32) {
        self.scale = (self.scale + self.scale * delta).clamp(0.01, 8.0);
    }

    pub fn draw(&mut self) {
        self.terrain.draw(self.scale, self.scroll_vector);
    }
}