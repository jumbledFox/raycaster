use image;
use nalgebra::Vector2;

// One byte for texture index
// One byte for kind
// One byte for any flags
pub struct Cell {
    pub texture_index: u8,
    pub kind: u8,
    pub flags: u8,
}

impl Cell {
    pub fn new(texture_index: u8, kind: u8, flags: u8) -> Cell {
        Cell { texture_index, kind, flags }
    }
}
/* 
Kinds             | Flags
0 - Nothing       | None
1 - Solid wall    | None
2 - Light         | 00 00 00 rgb colours
3 - Door          | 00 direction (NS, EW, SN, WE) 00 open type (slide left, right, up, down)
4 - Thin wall     | 0 direction
5 - Thick wall    | 0 direction
6 - Square Pillar | 
7 - Round Pillar  | 
8 - Diagonal      | 0 direction (TL to BR, TR to BL) // maybe make it solid
*/

pub struct Map {
    pub cells: Vec<Cell>,
    pub width : usize,
    pub height: usize,
}

impl Map {
    pub fn get(&self, index: usize) -> &Cell {
        return &self.cells[index];
    }

    pub fn load(image_path: String) -> Map {
        let img = image::open(image_path).unwrap().to_rgb8();
        let width  = img.width()  as usize;
        let height = img.height() as usize;

        let mut player_spawn = Vector2::new(0.0, 0.0);

        let mut cells: Vec<Cell> = Vec::with_capacity(width*height);
        
        for (i, p) in img.pixels().enumerate() {
            cells.push(match p.0 {
                // Light
                [255, 255,   0] => Cell::new(0, 2, 0b_11_11_11_00),
                // Solid - white
                [255, 255, 255] => Cell::new(0, 1, 0),
                // Thick wall NS
                [188,  96, 188] => Cell::new(0, 5, 0b0000000_0),
                // Thick wall EW
                [255, 128, 255] => Cell::new(0, 5, 0b0000000_1),
                // Square pillar
                [  0, 174, 255] => Cell::new(0, 6, 0b00000000),
                // Diagonal TL BR
                [  0, 128,   0] => Cell::new(0, 8, 0b0000000_0),
                // Diagonal TR BL
                [  0, 255,   0] => Cell::new(0, 8, 0b0000000_1),

                // Setting positions
                // Player position
                [255,   0, 255] => {
                    player_spawn.x = (i % width) as f64 + 0.5;
                    player_spawn.y = (i / width) as f64 + 0.5;
                    Cell::new(0, 0, 0)
                },
                // Nothing
                _ => Cell::new(0, 0, 0),
            });
        }
        Map {cells, width, height}
    }
}
