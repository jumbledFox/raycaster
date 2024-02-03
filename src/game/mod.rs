pub mod player;
pub mod light;

use nalgebra::Vector2;
use player::Player;
use light::Light;

use image;

pub struct Game {
    pub player: Player,
    pub lights: Vec<Light>,
    pub map: Vec<u8>,
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
                *m = 1;
            }
        }

        let lights: Vec<Light> = vec![Light::new(Vector2::new(0.0, 0.0), 3.0)];

        Game {
            player: Player::new(),
            lights,
            texture: texture.chunks_exact(4).map(|chunk| chunk.try_into().unwrap()).collect(),
            texture_size: (im.width() as usize, im.height() as usize),
            map, map_width, map_height,
        }
    }
}