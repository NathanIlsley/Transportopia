use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Tile {
    Grass,
}

#[derive(Clone, Copy)]
pub struct TerrainChunkConfig {
    pub chunk_tile_size: u32,
    pub chunks_from_centre: u32,
    pub chunk_texture_scale: f32,
}

impl Default for TerrainChunkConfig {
    fn default() -> Self {
        Self {
            chunk_tile_size: 8,
            chunks_from_centre: 3,
            chunk_texture_scale: 1.0,
        }
    }
}

struct Chunk {
    chunk_pos: IVec2,
    built: bool,
    render_target: RenderTarget,
    rt_camera: Camera2D,
    chunk_tile_size: u32,
    tile_dim: Vec2,
    world_texture_size: Vec2,
    texture_scale: f32,
}

impl Chunk {
    fn new(
        chunk_pos: IVec2,
        chunk_tile_size: u32,
        tile_dim: Vec2,
        texture_scale: f32,
    ) -> Self {
        let world_texture_size = vec2(
            (chunk_tile_size as f32 * tile_dim.x * 2.0).ceil().max(1.0),
            (chunk_tile_size as f32 * tile_dim.y * 2.0).ceil().max(1.0),
        );
        let texture_scale = texture_scale.max(0.125);
        let texture_width = (world_texture_size.x * texture_scale).ceil().max(1.0) as u32;
        let texture_height = (world_texture_size.y * texture_scale).ceil().max(1.0) as u32;

        let render_target = render_target(texture_width, texture_height);
        render_target.texture.set_filter(FilterMode::Nearest);
        let cam_render_target = render_target.clone();

        Self {
            chunk_pos,
            built: false,
            render_target,
            rt_camera: Camera2D {
                render_target: Some(cam_render_target),
                ..Camera2D::from_display_rect(Rect::new(
                    0.0,
                    0.0,
                    texture_width as f32,
                    texture_height as f32,
                ))
            },
            chunk_tile_size,
            tile_dim,
            world_texture_size,
            texture_scale,
        }
    }

    fn is_built(&self) -> bool {
        self.built
    }

    fn build(&mut self, grass: &Texture2D) {
        set_camera(&self.rt_camera);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

        let tile_size = vec2(grass.width(), grass.height()) * self.texture_scale;
        let y_offset = (self.chunk_tile_size as f32 - 1.0) * self.tile_dim.y * self.texture_scale;

        for tile_x in 0..self.chunk_tile_size as i32 {
            for tile_y in 0..self.chunk_tile_size as i32 {
                draw_texture_ex(
                    grass,
                    (tile_x + tile_y) as f32 * self.tile_dim.x * self.texture_scale,
                    (tile_y - tile_x) as f32 * self.tile_dim.y * self.texture_scale + y_offset,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(tile_size),
                        ..Default::default()
                    },
                );
            }
        }

        set_default_camera();
        self.built = true;
    }

    fn draw_to_screen(&self, scale: f32, scroll_vector: Vec2) {
        let chunk_width_in_tiles = self.chunk_tile_size as f32;
        let world_x = (self.chunk_pos.x as f32 * chunk_width_in_tiles
            + self.chunk_pos.y as f32 * chunk_width_in_tiles)
            * self.tile_dim.x;
        let world_y = (self.chunk_pos.y as f32 * chunk_width_in_tiles
            - self.chunk_pos.x as f32 * chunk_width_in_tiles)
            * self.tile_dim.y
            - (self.chunk_tile_size as f32 - 1.0) * self.tile_dim.y;

        draw_texture_ex(
            &self.render_target.texture,
            screen_width() / 2.0 + world_x * scale + scroll_vector.x,
            screen_height() / 2.0 + world_y * scale + scroll_vector.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    self.world_texture_size.x * scale,
                    self.world_texture_size.y * scale,
                )),
                ..Default::default()
            },
        );
    }
}

pub struct TileManager {
    grass: Texture2D,
    chunks: Vec<Vec<Chunk>>,
}

impl TileManager {
    pub fn new(grass: Texture2D, config: TerrainChunkConfig) -> Self {
        let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);
        let mut chunks = Vec::new();
        let chunk_radius = config.chunks_from_centre as i32;

        for chunk_x in -chunk_radius..=chunk_radius {
            let mut chunk_column = Vec::new();
            for chunk_y in -chunk_radius..=chunk_radius {
                chunk_column.push(Chunk::new(
                    ivec2(chunk_x, chunk_y),
                    config.chunk_tile_size,
                    tile_dim,
                    config.chunk_texture_scale,
                ));
            }
            chunks.push(chunk_column);
        }

        Self { grass, chunks }
    }

    pub fn draw(&mut self, scale: f32, scroll_vector: Vec2) {
        for column in &mut self.chunks {
            for chunk in column {
                if !chunk.is_built() {
                    chunk.build(&self.grass);
                }

                chunk.draw_to_screen(scale, scroll_vector);
            }
        }
    }
}