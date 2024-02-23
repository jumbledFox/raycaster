use std::collections::HashMap;

use image;
use nalgebra::{point, Vector2};

use super::collision::Segment;

// One byte for texture index
// One byte for kind
// One byte for any flags
pub struct Cell {
    pub kind: u8,
    pub flags: u8,
    pub texture_index: u8,
}

impl Cell {
    pub fn new(kind: u8, flags: u8, texture_index: u8) -> Cell {
        Cell { kind, flags, texture_index }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DoorState {
    Closed,
    Open(f64),
    Closing(f64),
    Opening(f64),
}
/* 
Kinds             | Flags
0 - Nothing       | None
1 - Solid wall    | None
2 - Light         | 00 00 00 rgb colours
3 - Door          | 0 (x/y), 0 (flipped), 00 type (slide, elevator1, elevator2, hinge (how the fuck will i do that then...))
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
    pub doors: HashMap<usize, DoorState>,
    pub lightmap: Vec<u8>,

    pub collision: Vec<Segment>,
}

impl Map {
    pub fn coord_to_index(&self, x: &usize, y: &usize) -> usize {
        y * self.width + x
    }
    pub fn index_to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn get(&self, index: usize) -> &Cell {
        return &self.cells[index];
    }

    pub fn load(image_path: String) -> Map {
        let img = image::open(image_path).unwrap().to_rgb8();
        let width  = img.width()  as usize;
        let height = img.height() as usize;

        let mut player_spawn = Vector2::new(0.0, 0.0);

        let mut cells: Vec<Cell> = Vec::with_capacity(width*height);
        let mut doors = HashMap::new();

        for (i, p) in img.pixels().enumerate() {
            cells.push(match p.0 {
                // Solid - white
                [255, 255, 255] => Cell::new(1, 0, 0),
                // Light
                [255, 255,   0] => Cell::new(2, 0b_11_11_11_00, 0),
                // Door NS
                [127,  81,  25] => {
                    doors.insert(i, DoorState::Closed);
                    Cell::new(3, 0b0000_00_00, 1)
                }
                // Door WE
                [204, 130,  40] => {
                    doors.insert(i, DoorState::Closed);
                    Cell::new(3, 0b0000_00_11, 1)
                }
                // Elevator door NS
                [119, 119, 119] => {
                    doors.insert(i, DoorState::Closed);
                    Cell::new(3, 0b0000_01_00, 2)
                }
                // Thin wall Map
                [177, 255,  61] => Cell::new(4, 0b0000000_1, 3),
                // Thick wall NS
                [188,  96, 188] => Cell::new(5, 0b0000000_0, 0),
                // Thick wall EW
                [255, 128, 255] => Cell::new(5, 0b0000000_1, 0),
                // Square pillar
                [  0, 174, 255] => Cell::new(6, 0b00000000,  0),
                // Round pillar
                [  0,   0, 255] => Cell::new(7, 0b00000000,  0),
                // Diagonal TL BR
                [  0, 128,   0] => Cell::new(8, 0b0000000_0, 0),
                // Diagonal TR BL
                [  0, 255,   0] => Cell::new(8, 0b0000000_1, 0),

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
        let mut m = Map {cells, width, height, doors, lightmap: vec![], collision: vec![]};
        m.calculate_lightmap();
        m.calculate_collision();
        m
    }

    pub fn calculate_lightmap(&mut self) {
        self.lightmap = vec![0; self.width*self.height];
        // Find where all of the light are
        let light_positions: Vec<usize> = self.cells.iter()
            .enumerate()
            .filter(|(_, item)| item.kind == 2)
            .map(|(index, _)| index)
            .collect();
        
        // For each light...
        for lp in light_positions {
            let mut light_level = 16;
            
            // Which cells this light has checked
            let mut done: Vec<usize> = vec![];
            // Positions we're currently checking
            let mut fronteir: Vec<usize> = vec![lp];
            // Holds all of the neighbours we're gonna do next.
            let mut neighbours : Vec<usize> = vec![];

            while light_level > 0 {
                for &index in &fronteir {
                    // Make sure we only do each cell once per light
                    if done.contains(&index) {
                        continue;
                    }
                    done.push(index);

                    // Set the light level of the cell (if it's higher than the old one)
                    if self.lightmap[index] < light_level {
                        self.lightmap[index] = light_level;
                    }

                    // Check and add all neighbours of this cell
                    let neighbour_offsets = [index.checked_add(1), index.checked_sub(1), index.checked_add(self.width), index.checked_sub(self.width)];
                    for (i, neighbour_index) in neighbour_offsets.iter().enumerate() {
                        // Skip if the neighbour isn't valid
                        if neighbour_index.is_none() { continue; }
                        // Make sure we don't go off an edge
                        if match i {
                            /* Right */ 0 => { index % self.width == self.width-1 }
                            /* Left  */ 1 => { index % self.width == 0 }
                            /* Down  */ 2 => { index / self.width == self.height-1 }
                            /* Up    */ _ => { index / self.width == 0 }
                        } { continue; }
                        // Skip if the neighbour is solid
                        if self.cells.get(neighbour_index.unwrap()).unwrap().kind == 1 { continue; }
                        neighbours.push(neighbour_index.unwrap());
                    } 
                }
                // Remove all duplicate neighbours
                neighbours.sort_unstable();
                neighbours.dedup();
                // Make it so the new frontier is made up of the neighbours we just found
                fronteir = std::mem::replace(&mut neighbours, fronteir);
                // Clear the neighbours for next run
                neighbours.clear();

                light_level -= 1;
            }
        }
    }

    fn calculate_collision(&mut self) {
        self.collision = vec![
            [point![5.0, 3.0], point![5.0, 2.0]], 
            [point![5.0, 2.0], point![5.0, 1.0]], 
            [point![5.0, 1.0], point![4.0, 1.0]], 
            [point![4.0, 1.0], point![3.0, 1.0]], 
            [point![3.0, 1.0], point![2.0, 1.0]], 
            [point![2.0, 1.0], point![1.0, 2.0]], 
            [point![1.0, 2.0], point![1.0, 3.0]], 
            [point![1.0, 3.0], point![1.0, 4.0]], 
            [point![1.0, 4.0], point![2.0, 5.0]], 
            [point![2.0, 5.0], point![3.0, 6.0]], 
            [point![3.0, 6.0], point![4.0, 6.0]], 
            [point![4.0, 6.0], point![4.0, 7.0]], 
            [point![5.0, 7.0], point![5.0, 6.0]], 
            [point![5.0, 6.0], point![6.0, 6.0]], 
            [point![2.0, 2.0], point![2.0, 2.0]], 
            [point![3.25, 2.25], point![3.25, 2.75]], 
            [point![3.75, 2.25], point![3.75, 2.75]], 
            [point![3.25, 2.25], point![3.75, 2.25]], 
            [point![3.25, 2.75], point![3.75, 2.75]], 
        ]
    }
}
