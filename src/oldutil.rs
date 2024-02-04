use nalgebra::Vector2;

use crate::{WIDTH, game::Game};

use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum RaycastSide { X, Y }

// Shoots a raycast from a position and a direction and returns what it hit (as an index in the map),
// the hit point, how far away it was, and if it hit x or y!
pub fn raycast(game: &Game, start_pos: Vector2<f64>, dir: Vector2<f64>, max_dist: f64, tell_info: bool) -> Option<(usize, Vector2<f64>, f64, RaycastSide)> {
    // If the ray is out of bounds, don't bother.
    if  start_pos.x < 0.0 || start_pos.x > game.map_width  as f64 ||
        start_pos.y < 0.0 || start_pos.y > game.map_height as f64 {
        return None;
    }
    // DDA algorithm
    
    // Where the ray will end
    let end_pos = start_pos + dir * max_dist;
    // Which box of the map we're in
    let mut map_pos: Vector2<isize> = Vector2::new(
        start_pos.x as isize,
        start_pos.y as isize);
    // Accumulated columns and rows of the length of the ray, used to compare.
    let mut ray_length_1d = Vector2::new(0.0, 0.0);
    // 1 or -1 for each direction
    let step_x: isize;
    let step_y: isize;
    // Length of side in triangle formed by ray if the other side is length 1 (from one cell to the next)
    let step_size = Vector2::new(
        f64::sqrt(1.0 + (dir.y / dir.x) * (dir.y / dir.x)),
        f64::sqrt(1.0 + (dir.x / dir.y) * (dir.x / dir.y)),
    );

    // Set step and calculate from position to first intersection point
    if dir.x < 0.0 {
        step_x = -1;
        ray_length_1d.x = (start_pos.x - map_pos.x as f64) * step_size.x;
    } else {
        step_x =  1;
        ray_length_1d.x = ((map_pos.x + 1) as f64 - start_pos.x) * step_size.x;
    }
    if dir.y < 0.0 {
        step_y = -1;
        ray_length_1d.y = (start_pos.y - map_pos.y as f64) * step_size.y;
    } else {
        step_y =  1;
        ray_length_1d.y = ((map_pos.y + 1) as f64 - start_pos.y) * step_size.y;
    }

    let mut distance: f64 = 0.0;
    let mut side = RaycastSide::X;
    
    let mut tile_found = false;
    while !tile_found && distance < max_dist {
        // Move along either X or Y
        if ray_length_1d.x < ray_length_1d.y {
            map_pos.x += step_x;
            distance = ray_length_1d.x;
            ray_length_1d.x += step_size.x;
            side = RaycastSide::X;
        } else {
            map_pos.y += step_y;
            distance = ray_length_1d.y;
            ray_length_1d.y += step_size.y;
            side = RaycastSide::Y;
        }
        // If out of bounds, stop checking
        if  map_pos.x > game.map_width as isize - 1 || map_pos.y > game.map_height as isize - 1 ||
            map_pos.x < 0 || map_pos.y < 0 {
            break;
        }
        
        // We know it's not out of bounds, so let's get where it is.
        let x_pos: usize = map_pos.x.try_into().unwrap();
        let y_pos: usize = map_pos.y.try_into().unwrap();
        // TODO: Make this bit better
        // If the end pos is in the map pos
        if  end_pos.x >= 0.0 && end_pos.x < game.map_width as f64 &&
            end_pos.y >= 0.0 && end_pos.y < game.map_height as f64 {
                if  end_pos.x.floor() as usize == x_pos && 
                    end_pos.y.floor() as usize == y_pos {
                return None
            }
        }
        
        // // Depending on the tile 
        let tile = *game.map.get(y_pos * game.map_width + x_pos).unwrap_or(&0);
        match tile {
            // It's not solid, so we don't care
            0 | 1 => (),
            // It's a thin wall! We MIGHT have found a tile
            // N/S
            5 => {
                let current_pos = start_pos + dir * distance;
                let current_pos = Vector2::new(current_pos.x.rem_euclid(1.0), current_pos.y.rem_euclid(1.0));

                // Work out the next cell it'll hit
                let next_pos = match ray_length_1d.x < ray_length_1d.y {
                    true  => start_pos + dir * (distance + step_size.x - current_pos.y),
                    false => start_pos + dir * (distance + step_size.y - current_pos.x),
                };
                // Make them local
                // let current_pos = Vector2::new(current_pos.x.rem_euclid(1.0), current_pos.y.rem_euclid(1.0));
                // let next_pos    = Vector2::new(next_pos   .x.rem_euclid(1.0), next_pos   .y.rem_euclid(1.0));
                if tell_info {
                    // match ray_length_1d.x < ray_length_1d.y {
                    //     true  => println!("x: {:?} {:?}", step_size.x, (distance + step_size.x)),
                    //     false => println!("y: {:?} {:?}", step_size.y, (distance + step_size.y)),
                    // };
                    println!("Current: [{:.2}, {:.2}] Next: [{:.2}, {:.2}]", current_pos.x, current_pos.y, next_pos.x, next_pos.y);
                }
            },
            // E/W
            6 => {
                
            }
            // Otherwise it must be solid
            _ => tile_found = true,
        }
    }
    // For correcting bulge, instead of this method, which doesn't seem to work:
    // https://lodev.org/cgtutor/raycasting.html
    // i multiply the distance by cos of the angle, as shown here:
    // https://www.permadi.com/tutorial/raycast/rayc8.html

    match tile_found && distance < max_dist {
        true =>  {
            let perp_dist = distance*dir.angle(&game.player.dir).cos();
            Some((map_pos.y as usize * game.map_width + map_pos.x as usize, start_pos + dir*perp_dist, perp_dist, side))
        }
        false => None
    }
}