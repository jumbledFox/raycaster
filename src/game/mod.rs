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
                Texture::from_file("res/wall1.png"),
                Texture::from_file("res/door2.png"),
                Texture::from_file("res/elevator.png"),
            ],
        }
    }
}