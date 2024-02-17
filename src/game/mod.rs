pub mod player;
pub mod map;
pub mod texture;

use nalgebra::Vector2;

use player::Player;
use map::Map;

use image;

use self::{map::Cell, texture::Texture};

pub struct Game {
    pub player: Player,
    pub map: Map,
    pub textures: Vec<Texture>,
    pub lightmap: Vec<u8>,
}

impl Game {
    pub fn coord_to_index(&self, x: &usize, y: &usize) -> usize {
        y * self.map.width + x
    }
    pub fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.map.width, index / self.map.width)
    }

    pub fn new() -> Game {
        Game {
            player: Player::new(Vector2::new(13.0, 4.0)),
            map: Map::load(String::from("res/map3.png")),
            textures: vec![
                Texture::from_file("res/testtexture.png"),
                Texture::from_file("res/doormetal.png"),
                Texture::from_file("res/elevator.png"),
            ],
            lightmap: vec![16; 9999],
        }
    }

    // TODO: Maybe implement something like this:
    // https://www.reddit.com/r/Minecraft/comments/8swb5s/comment/e13uu9m/?utm_source=share&utm_medium=web2x&context=3
    pub fn calculate_lightmap(&mut self) {
        self.lightmap = vec![0; self.map.width*self.map.width];
        // Find where all of the light are
        let light_positions: Vec<usize> = self.map.cells.iter()
            .enumerate()
            .filter(|(_, item)| item.kind == 2)
            .map(|(index, _)| index)
            .collect();
        // For each light in the scene
        for lp in light_positions {
            // Flood fill routine
            let mut light_level = 16;
            // TODO: Maybe use something like a hashset where each element is unique
            let mut done: Vec<usize> = vec![];
            let mut current_positions: Vec<usize> = vec![lp];
            let mut position_buffer  : Vec<usize> = vec![];

            while light_level > 0 {
                // For each position
                for index in &mut current_positions {
                    // Skip if it's been done before
                    if done.contains(&index) { continue; }
                    else { done.push(*index); }

                    // Set the light level (if it's higher than the old one)
                    if self.lightmap[*index] < light_level {
                        self.lightmap[*index] = light_level;
                    }

                    // Add all of the neighbours to the position buffer
                    let coord = self.index_to_coord(*index);
                    let neighbour_offsets = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for n in neighbour_offsets {
                        let n_x = coord.0.checked_add_signed(n.0);
                        let n_y = coord.1.checked_add_signed(n.1);
                        // Skip if neighbour out of bounds
                        if  n_x.is_none() || n_y.is_none() ||
                            n_x.is_some_and(|j| j >= self.map.width) ||
                            n_y.is_some_and(|j| j >= self.map.height)
                            { continue; }

                        let neighbour_index = self.coord_to_index(&n_x.unwrap(), &n_y.unwrap());
                        // Skip if the neighbour is solid
                        match self.map.get(neighbour_index).kind {
                            1 => continue,
                            // TODO: Give shapes some kind of flag for allowing light through or not
                            0 | 2 | _ => (),
                        };
                        // Add to position buffer (may contain duplicates, so we'll have to deal with that later)
                        position_buffer.push(neighbour_index);
                    }
                }                
                // Remove duplicates
                position_buffer.sort_unstable();
                position_buffer.dedup();
                // Swap and clear buffers
                position_buffer = std::mem::replace(&mut current_positions, position_buffer);
                position_buffer.clear();

                light_level -= 1;
            }
        }
    }
}