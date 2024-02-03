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
            let mut done: Vec<(usize, usize)> = vec![];
            let mut current_positions: Vec<(usize, usize)> = vec![self.index_to_coord(lp)];
            let mut position_buffer  : Vec<(usize, usize)> = vec![];

            while light_level > 0 {
                // For each position
                for coord in &mut current_positions {
                    let index = self.coord_to_index(coord);
                    // Skip if it's been done before
                    if done.contains(&coord) {
                        continue;
                    } else {
                        done.push(*coord);
                    }
                    // Set the light level
                    self.lightmap[index] = light_level;

                    // Add all of the neighbours to the position buffer
                    let c = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for n in c {
                        let n_x = coord.0.checked_add_signed(n.0);
                        let n_y = coord.1.checked_add_signed(n.1);
                        // Skip if neighbour out of bounds.
                        if  n_x.is_none() || n_y.is_none() ||
                            n_x.is_some_and(|j| j >= self.map_width) ||
                            n_y.is_some_and(|j| j >= self.map_height)
                            { continue; }
                        // Skip if the neighbour is solid.
                        // Add to position buffer (may contain duplicates, so we'll have to deal with that later)
                        position_buffer.push((n_x.unwrap(), n_y.unwrap()));
                    }
                }
                light_level -= 1;
                // Remove duplicates
                position_buffer.sort_unstable();
                position_buffer.dedup();
                // Swap and clear buffers 
                position_buffer = std::mem::replace(&mut current_positions, position_buffer);
                position_buffer.clear();
            }

            
            // let mut done_positions: Vec<usize> = vec![];
            // let x = lp % self.map_width;
            // let y = lp / self.map_width;
            // self.lightmap_set_neighbour(&mut done_positions, x, y, 16);
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

    fn lightmap_set_neighbour(&mut self, done_positions: &mut Vec<usize>, x: usize, y: usize, light_level: u8) {
        // Check each neighbour
        let c = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut next_ones: Vec<(usize, usize)> = vec![];
        for i in 0..4 {
            let x_pos = x.checked_add_signed(c[i].0);
            let y_pos = y.checked_add_signed(c[i].1);
            // Skip if neighbour out of bounds.
            if x_pos.is_none() || y_pos.is_none() ||
                x_pos.is_some_and(|j| j >= self.map_width) ||
                y_pos.is_some_and(|j| j >= self.map_height)
                { continue; }
            let neighbour_pos = y_pos.unwrap() * self.map_width + x_pos.unwrap();
            // Skip if the neighbour is solid
            if self.map[neighbour_pos] != 0 { continue; }
            // Set the light level and go to the next step (if there's enough light)
            if !done_positions.contains(&neighbour_pos) {
                done_positions.push(neighbour_pos);
                self.lightmap[neighbour_pos] = light_level;
                if light_level - 1 != 0 {
                    next_ones.push((x_pos.unwrap(), y_pos.unwrap()));
                }
            }
        }
        for n in next_ones {
            self.lightmap_set_neighbour(done_positions, n.0, n.1, light_level-1);
        }
    }
}