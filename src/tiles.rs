// use crate::structures::Structure;

const WORLD_SIZE: usize = 1024;

#[derive(Clone, Copy)]
pub enum Tile {
    Grass(),//&Structure
    // Sand(),
}

// // #[derive(Copy)]
// struct Tile {
//     ground_type: GroundType,
//     structures: Vec<Structure>,
// }

// impl Tile {
//     fn add_structure(&mut self, structure: Structure) {
//         match self {
//             Tile::Grass(tiles) => {
//                 tiles.push(structure);
//             }
//         }
//     }

//     fn get_structures(&self) -> &Vec<Structure> {
//         match self {
//             Tile::Grass(tiles) => tiles,
//         }
//     }
// }

pub struct TileManager {
    tiles: [[Tile; WORLD_SIZE * 2 + 1]; WORLD_SIZE * 2 + 1],
}

impl TileManager {
    pub fn new() -> Self {
        let tiles = [[Tile::Grass(); WORLD_SIZE * 2 + 1]; WORLD_SIZE * 2 + 1];

        Self { tiles }
    }

    pub fn get_tile(&self, x: isize, y: isize) -> Option<&Tile> {
        if x.abs() < WORLD_SIZE as isize && y.abs() < WORLD_SIZE as isize {
            Some(&self.tiles[(x + WORLD_SIZE as isize) as usize][(y + WORLD_SIZE as isize) as usize])
        } else {
            None
        }
    }

    pub fn is_grass(&self, x: isize, y: isize, tiles_from_centre: isize) -> Option<bool> {
        if x + tiles_from_centre < WORLD_SIZE as isize && y + tiles_from_centre < WORLD_SIZE as isize && x - tiles_from_centre >= -(WORLD_SIZE as isize) && y - tiles_from_centre >= -(WORLD_SIZE as isize) {
            let mut result = true;
            
            for i in x - tiles_from_centre..x + tiles_from_centre {
                for j in y - tiles_from_centre..y + tiles_from_centre {
                    if let Tile::Grass() = self.tiles[(i + WORLD_SIZE as isize) as usize][(j + WORLD_SIZE as isize) as usize] {} else {
                        result = false;
                        break;
                    }
                }
            }

            Some(result)
        } else {
            None
        }
    }

    pub fn set_tile_ground_type(&mut self, x: isize, y: isize, tile: Tile) {
        if x.abs() < WORLD_SIZE as isize && y.abs() < WORLD_SIZE as isize {
            self.tiles[(x + WORLD_SIZE as isize) as usize][(y + WORLD_SIZE as isize) as usize] = tile;
        }
    }
}