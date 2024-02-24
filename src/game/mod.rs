pub mod player;
pub mod map;
pub mod texture;
pub mod collision;

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
    pub fn new() -> Game {
        Game {
            // TODO: Make player position load from map
            player: Player::new(Vector2::new(13.0, 4.0)),
            map: Map::load(String::from("res/images/map3.png")),
            textures: vec![
                Texture::from_file("res/images/wall1.png"),
                Texture::from_file("res/images/door2.png"),
                Texture::from_file("res/images/elevator.png"),
                Texture::from_file("res/images/map3.png"),
            ],
        }
    }
}