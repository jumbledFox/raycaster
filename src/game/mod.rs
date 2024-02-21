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
            map: Map::load(String::from("res/map4.png")),
            textures: vec![
                Texture::from_file("res/wall1.png"),
                Texture::from_file("res/door2.png"),
                Texture::from_file("res/elevator.png"),
                Texture::from_file("res/map3.png"),
            ],
        }
    }
}