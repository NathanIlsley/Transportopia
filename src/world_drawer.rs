use macroquad::prelude::*;
use macroquad::telemetry::ZoneGuard;
use crate::structures::Structure;
use crate::tiles::Tile;
use crate::tiles::TileManager;

const CHUNK_RESOLUTION: f32 = 256.0;
const CHUNKS_FROM_CENTRE: usize = 8;//8;

pub struct Chunk {
    chunk_pos: Vec2,
    to_next_chunk: i32,
    tile_dim: Vec2,
    live_chunk: bool,
    built: bool,
    tiles: Vec<Vec<Tile>>,
}

impl Chunk {
    pub fn new(chunk_pos: Vec2, to_next_chunk: i32, tile_dim: Vec2, live_chunk: bool) -> Self {
        Self {
            chunk_pos,
            to_next_chunk,
            tile_dim,
            live_chunk,
            built: false,
            tiles: Vec::new(),
        }
    }

    pub fn is_built(&self) -> bool {
        self.built
    }

    pub fn set_live_chunk(&mut self, live_chunk: bool) {
        self.live_chunk = live_chunk;
    }

    pub fn is_live_chunk(&self) -> bool {
        self.live_chunk
    }

    pub fn build(&mut self, tile_manager: &TileManager) {
        let chunk_dist = self.to_next_chunk as i32 * 2 + 1;
        let chunk_centre = Vec2::new(self.chunk_pos.x as f32 * chunk_dist as f32, self.chunk_pos.y as f32 * chunk_dist as f32);
        
        let mut tiles = Vec::new();
        for i in (-self.to_next_chunk as i32 + chunk_centre.x as i32)..(self.to_next_chunk as i32 + 1 + chunk_centre.x as i32) {
            tiles.push(Vec::new());
            for j in ((-self.to_next_chunk as i32 + chunk_centre.y as i32)..(self.to_next_chunk + 1 + chunk_centre.y as i32)).rev() {
                tiles[(i + self.to_next_chunk - chunk_centre.x as i32) as usize].push(tile_manager.get_tile(i as isize, j as isize).unwrap_or(&Tile::Grass()).clone());
            }
        }

        self.tiles = tiles;
        self.built = true;
    }

    pub fn draw_to_render_target(&mut self, grass: &Texture2D) {
        let _z = ZoneGuard::new("Draw to render target");

        let x = (self.chunk_pos.x + CHUNKS_FROM_CENTRE as f32) * CHUNK_RESOLUTION as f32;
        let y = (self.chunk_pos.y + CHUNKS_FROM_CENTRE as f32) * CHUNK_RESOLUTION as f32;

        // Clear section being drawn
        draw_rectangle(x, y, CHUNK_RESOLUTION, CHUNK_RESOLUTION, Color::new(0.0, 0.0, 0.0, 0.0));

        let parameters = DrawTextureParams {
            dest_size: Some(Vec2::new(grass.width() / (self.to_next_chunk * 2 + 1) as f32, grass.height() / (self.to_next_chunk * 2 + 1) as f32)),
            ..Default::default()
        };

        for k in -self.to_next_chunk..self.to_next_chunk+1 {
        for l in -self.to_next_chunk..self.to_next_chunk+1 {
            draw_texture_ex(
                grass,
                x + CHUNK_RESOLUTION / 2.0 + (k - 1 - l) as f32 * self.tile_dim.x / (self.to_next_chunk * 2 + 1) as f32,
                y + CHUNK_RESOLUTION / 2.0 + (l - 1 + k) as f32 * self.tile_dim.y / (self.to_next_chunk * 2 + 1) as f32,
                WHITE,
                parameters.clone(),
            );
        }
        }
    }

    pub fn update(&mut self, grass: &Texture2D, update_target: &Texture2D, update_camera: &Camera2D, atlas_camera: &Camera2D) {
        if self.live_chunk {
            set_camera(update_camera);
            clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

            let parameters = DrawTextureParams {
                dest_size: Some(Vec2::new(grass.width() / (self.to_next_chunk * 2 + 1) as f32, grass.height() / (self.to_next_chunk * 2 + 1) as f32)),
                ..Default::default()
            };

            for k in -self.to_next_chunk..self.to_next_chunk+1 {
            for l in -self.to_next_chunk..self.to_next_chunk+1 {
                draw_texture_ex(
                    grass,
                    CHUNK_RESOLUTION / 2.0 + (k - 1 - l) as f32 * self.tile_dim.x / (self.to_next_chunk * 2 + 1) as f32,
                    CHUNK_RESOLUTION / 2.0 + (l - 1 + k) as f32 * self.tile_dim.y / (self.to_next_chunk * 2 + 1) as f32,
                    WHITE,
                    parameters.clone(),
                );
            }
            }

            set_camera(atlas_camera);

            draw_texture_ex(
                update_target,
                (self.chunk_pos.x + CHUNKS_FROM_CENTRE as f32) * CHUNK_RESOLUTION as f32,
                (self.chunk_pos.y + CHUNKS_FROM_CENTRE as f32) * CHUNK_RESOLUTION as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(CHUNK_RESOLUTION, CHUNK_RESOLUTION)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn draw_to_screen(&self, chunk_atlas_tex: &Texture2D, scale: f32, scroll_vector: &Vec2, parameters: DrawTextureParams) {
        draw_texture_ex(
            chunk_atlas_tex,
            screen_width() / 2.0 - 1.0 * chunk_atlas_tex.width() / 2.0 * scale + (self.chunk_pos.x - self.chunk_pos.y) * self.tile_dim.x * scale + scroll_vector.x + self.chunk_pos.x,
            screen_height() / 2.0 - 1.0 * chunk_atlas_tex.height() / 2.0 * scale + (self.chunk_pos.y + self.chunk_pos.x) * self.tile_dim.y * scale + scroll_vector.y + self.chunk_pos.y,
            WHITE,
            parameters,
        );
    }
}

pub struct WorldDrawer {
    scale: f32,
    // Number of tiles to draw in each direction from the centre on the render target
    to_next_chunk: i32,
    grass: Texture2D,
    tile_dim: Vec2,

    chunks: Vec<Vec<Chunk>>,
    chunks_atlas: RenderTarget,
    atlas_camera: Camera2D,
    chunk_update_target: RenderTarget,
    update_camera: Camera2D,

    scroll_vector: Vec2,
    structures: Vec<Structure>,
}

impl WorldDrawer {
    pub async fn new(grass_tex: &str, structures: Vec<Structure>) -> Self {
        let grass: Texture2D = load_texture(grass_tex).await.unwrap();
        grass.set_filter(FilterMode::Nearest);

        let to_next_chunk = 10;

        // x and y dimensions of the base game tile measured from the centre of the tile to each corner
        let tile_dim = vec2(grass.width() / 2.0, grass.height() / 2.0);

        let chunks_atlas = render_target(CHUNK_RESOLUTION as u32 * (2.0 * CHUNKS_FROM_CENTRE as f32 + 1.0) as u32, CHUNK_RESOLUTION as u32 * (2.0 * CHUNKS_FROM_CENTRE as f32 + 1.0) as u32);
        chunks_atlas.texture.set_filter(FilterMode::Nearest);
        let atlas_camera = Camera2D {
            render_target: Some(chunks_atlas.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, CHUNK_RESOLUTION * (2.0 * CHUNKS_FROM_CENTRE as f32 + 1.0), CHUNK_RESOLUTION * (2.0 * CHUNKS_FROM_CENTRE as f32 + 1.0)))
        };

        let chunk_update_target = render_target(CHUNK_RESOLUTION as u32, CHUNK_RESOLUTION as u32);
        chunk_update_target.texture.set_filter(FilterMode::Nearest);
        let update_camera = Camera2D {
            render_target: Some(chunk_update_target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, CHUNK_RESOLUTION, CHUNK_RESOLUTION))
        };

        let mut chunks = Vec::new();
        for i in -(CHUNKS_FROM_CENTRE as i32)..CHUNKS_FROM_CENTRE as i32 + 1 {
            chunks.push(Vec::new());
            for j in -(CHUNKS_FROM_CENTRE as i32)..CHUNKS_FROM_CENTRE as i32 + 1 {
                chunks[(i + CHUNKS_FROM_CENTRE as i32) as usize].push(Chunk::new(vec2(i as f32, j as f32), to_next_chunk, tile_dim, if i == 0 {true} else {false})); 
            }
        }

        Self {
            scale: 0.75,
            to_next_chunk,
            grass,
            tile_dim,

            chunks,
            chunks_atlas,
            atlas_camera,
            chunk_update_target,
            update_camera,

            scroll_vector: vec2(0.0, 0.0),
            structures,
        }
    }

    pub fn change_scroll(&mut self, delta: Vec2) {
        // let diagonal_tile_dim = ((self.tile_dim.x / 2.0).powi(2) + (self.tile_dim.y / 2.0).powi(2)).sqrt() * self.scale / self.to_next_chunk as f32;
        // let diagonal_dist_to_next_chunk = ((self.chunks[CHUNKS_FROM_CENTRE + 1][CHUNKS_FROM_CENTRE].chunk_pos.x * (self.to_next_chunk * 2 + 1) as f32 * self.tile_dim.x * self.scale - self.scroll_vector.x).powi(2)
        //                                          + (self.chunks[CHUNKS_FROM_CENTRE + 1][CHUNKS_FROM_CENTRE].chunk_pos.y * (self.to_next_chunk * 2 + 1) as f32 * self.tile_dim.y * self.scale - self.scroll_vector.y).powi(2)).sqrt();

        // println!("{}, {}", self.chunks[CHUNKS_FROM_CENTRE + 1][CHUNKS_FROM_CENTRE].chunk_pos.x, diagonal_dist_to_next_chunk);
        // if diagonal_dist_to_next_chunk < diagonal_tile_dim * (self.to_next_chunk * 2 + 1) as f32 / 2.0 {
        //     for i in 0..self.chunks.len() {
        //         let prev_max_y_pos = self.chunks[CHUNKS_FROM_CENTRE * 2 + 1][i].chunk_pos.y as f32;
        //         self.chunks[i].push(Chunk::new(vec2(i as f32, prev_max_y_pos), self.to_next_chunk, self.tile_dim, false));
        //         self.chunks[i].remove(0);
        //     }
        // }

        self.scroll_vector += delta;
    }

    pub fn change_zoom(&mut self, delta: i32) {
        if self.to_next_chunk - delta >= 0 && self.to_next_chunk - delta <= 10 {
            self.to_next_chunk -= delta;
        }
    }

    pub fn draw(&mut self, tile_manager: &TileManager) {
        set_camera(&self.atlas_camera);
        // clear_background(BLUE);
        for i in 0..self.chunks.len() {
        for j in (0..self.chunks[i].len()).rev() {
            if !self.chunks[i][j].is_built() {
                self.chunks[i][j].build(tile_manager);
                self.chunks[i][j].draw_to_render_target(&self.grass);
            } else {
                self.chunks[i][j].update(&self.grass, &self.chunk_update_target.texture, &self.update_camera, &self.atlas_camera);
            }
        }
        }
        
        set_default_camera();
        
        for i in 0..self.chunks.len() {
        for j in (0..self.chunks[i].len()).rev() {
            let parameters = DrawTextureParams {
                dest_size: Some(Vec2::new(CHUNK_RESOLUTION * self.scale, CHUNK_RESOLUTION * self.scale)),
                source: Some(Rect::new(i as f32 * CHUNK_RESOLUTION, j as f32 * CHUNK_RESOLUTION, CHUNK_RESOLUTION, CHUNK_RESOLUTION)),
                ..Default::default()
            };
            self.chunks[i][j].draw_to_screen(&self.chunks_atlas.texture, self.scale, &self.scroll_vector, parameters);
        }
        }
        
        for structure in &self.structures {
            structure.draw(vec2(0.0, 0.0), &(self.scale / (self.to_next_chunk * 2 + 1) as f32), &self.tile_dim, &self.scroll_vector);
            structure.draw(vec2(115.0, 115.0), &(self.scale / (self.to_next_chunk * 2 + 1) as f32), &self.tile_dim, &self.scroll_vector);
            structure.draw(vec2(10.0, 10.0), &(self.scale / (self.to_next_chunk * 2 + 1) as f32), &self.tile_dim, &self.scroll_vector);
        }
        
    }
}