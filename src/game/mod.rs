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
            let mut done_positions: Vec<usize> = vec![];
            let x = lp % self.map_width;
            let y = lp / self.map_width;
            self.lightmap_set_neighbour(&mut done_positions, x, y, 16);
        }
        for y in self.lightmap.chunks_exact(self.map_width) {
            for x in y {
                print!("{:02} ", x);
            }
            println!("");
        }
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