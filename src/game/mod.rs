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
    pub fn coord_to_index(&self, x: &usize, y: &usize) -> usize {
        y * self.map_width + x
    }
    pub fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.map_width, index / self.map_width)
    }

    pub fn new() -> Game {
        let im = image::open("res/bricks.png").unwrap().to_rgba8();
        let texture: Vec<u8> = im.clone().into_raw();

        let map_info = Game::load_map(String::from("res/map2.png"));
        let map = map_info.0;
        let map_width  = map_info.1;
        let map_height = map_info.2;
        /*
        let map_width  = 30;
        let map_height = 30;
        let mut map = vec![0; map_width*map_height];
        // Add edges
        for (i, m) in map.iter_mut().enumerate() {
            if i/map_width == 0 || i/map_width == map_height-1 || i%map_width == 0 || i%map_width == map_width-1 {
                *m = 2;
            }
        }
        // Add lights
        map[map_width*2+2] = 1;
        // Random lights
        // for i in 0..map.len() {
        //     if i < map_width || i > (map_width*map_height)-map_width { continue; }
        //     if i % map_width == 0 || i % map_width == map_width - 1 { continue; }
        //     let mut rng = rand::thread_rng();
        //     map[i] = match rng.gen_range(0..10) == 0 {
        //         true  => 1,
        //         false => 0,
        //     };
        // }
        // map[map_width*2+2] = 1;
        // map[map_width*3-3] = 1;
        */

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

    fn load_map(image_path: String) -> (Vec<u8>, usize, usize) {
        let im = image::open(image_path).unwrap().to_rgb8();
        let width  = im.width()  as usize;
        let height = im.height() as usize;
        let mut map = vec![0; width*height];
        for (i, p) in im.pixels().enumerate() {
            map[i] = match p.0 {
                [255, 255, 255] => 2, // wall
                [255, 255,   4] => 1, // light
                [254,   0,   0] => 3, // wall - red
                [190, 190, 190] => 4, // wall - orange
                [  4, 126,   0] => 5, // thin wall -
                [  6, 255,   4] => 6, // thin wall |
                [0xff, 0x00, 0xdc] => 7, // Cylinder
                _ => 0,
            };
            match p.0 {
                [254,   0, 255] => {},
                _ => {}
            }
        }
        (map, width, height)
    }

    // TODO: Maybe implement something like this:
    // https://www.reddit.com/r/Minecraft/comments/8swb5s/comment/e13uu9m/?utm_source=share&utm_medium=web2x&context=3
    pub fn calculate_lightmap(&mut self) {
        self.lightmap = vec![0; self.map_width*self.map_height];
        // Find where all of the light are
        let light_positions: Vec<usize> = self.map.iter()
            .enumerate()
            .filter(|(_, item)| **item == 1)
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
                            n_x.is_some_and(|j| j >= self.map_width) ||
                            n_y.is_some_and(|j| j >= self.map_height)
                            { continue; }

                        let neighbour_index = self.coord_to_index(&n_x.unwrap(), &n_y.unwrap());
                        // Skip if the neighbour is solid (or a thin wall)
                        match self.map[neighbour_index] {
                            0 | 1 | 5 | 6 => (),
                            _ => continue
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