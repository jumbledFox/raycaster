pub mod player;

use nalgebra::Vector2;
use player::Player;

use image;

pub struct Game {
    pub player: Player,
    pub map: Vec<u8>,
    pub lightmap: Vec<u8>,
    pub texture: Vec<[u8; 4]>,
    pub texture_size: (usize, usize),
    pub map_width: usize,
    pub map_height: usize,
}

impl Game {
    pub fn new() -> Game {
        // let im = image::open("res/tarkus.png").unwrap().to_rgba8();
        // let im = image::open("res/example_img.png").unwrap().to_rgba8();
        let im = image::open("res/bricks.png").unwrap().to_rgba8();
        let texture: Vec<u8> = im.clone().into_raw();

        let map_width = 15;
        let map_height = 15;
        let mut map = vec![0; map_width*map_height];
        // Add edges
        for (i, m) in map.iter_mut().enumerate() {
            if i/map_width == 0 || i/map_width == map_height-1 || i%map_width == 0 || i%map_width == map_width-1 {
                *m = 2;
            }
        }
        // Add light
        map[map_width*2+2] = 1;

        let mut g = Game {
            player: Player::new(),
            texture: texture.chunks_exact(4).map(|chunk| chunk.try_into().unwrap()).collect(),
            texture_size: (im.width() as usize, im.height() as usize),
            map, map_width, map_height,
            lightmap: vec![],
        };
        g.calculate_lightmap();
        g
    }

    pub fn calculate_lightmap(&mut self) {
        self.lightmap = vec![0; self.map_width*self.map_height];
        // Find all light positions
        let light_positions: Vec<usize> = self.map.iter()
            .enumerate()
            .filter(|(_, item)| **item == 1)
            .map(|(index, _)| index)
            .collect();
        for lp in light_positions {
            let mut light_level = 16;
            let mut done: Vec<usize> = vec![];
            let mut current_positions: Vec<usize> = vec![lp];
            let mut position_buffer  : Vec<usize> = vec![];

            while light_level > 0 {
                // For each position
                for index in &mut current_positions {
                    // Skip if it's been done before
                    if done.contains(&index) { continue; }
                    else { done.push(*index); }

                    // Set the light level
                    self.lightmap[*index] = (self.lightmap[*index] + light_level).min(16);

                    // Add all of the neighbours to the position buffer
                    let coord = self.index_to_coord(*index);
                    let neighbour_offsets = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for n in neighbour_offsets {
                        let n_x = coord.0.checked_add_signed(n.0);
                        let n_y = coord.1.checked_add_signed(n.1);
                        // Skip if neighbour out of bounds
                        if  n_x.is_none() || n_y.is_none() ||
                            n_x.is_some_and(|j| j >= self.map_width) ||
                            n_y.is_some_and(|j| j >= self.map_height)
                            { continue; }

                        let neighbour_index = self.coord_to_index(&(n_x.unwrap(), n_y.unwrap()));
                        // Skip if the neighbour is solid
                        if self.map[neighbour_index] != 0 && self.map[neighbour_index] != 1 {
                            continue;
                        }
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
        for y in self.lightmap.chunks_exact(self.map_width) {
            for x in y {
                print!("{:02} ", x);
            }
            println!("");
        }
    }

    pub fn coord_to_index(&self, c: &(usize, usize)) -> usize {
        c.1 * self.map_width + c.0
    }
    pub fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.map_width, index / self.map_width)
    }
}