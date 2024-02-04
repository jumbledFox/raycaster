use nalgebra::Vector2;

use crate::{WIDTH, game::Game};

use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum RaycastSide { X, Y }

// Shoots a raycast from a position and a direction and returns what it hit (as an index in the map),
// the hit point, how far away it was, and if it hit x or y!
pub fn raycast(game: &Game, start_pos: Vector2<f64>, dir: Vector2<f64>, max_dist: f64) -> Option<(usize, Vector2<f64>, f64, RaycastSide)> {
    // If the ray is out of bounds, don't bother.
    if  start_pos.x < 0.0 || start_pos.x > game.map_width  as f64 ||
        start_pos.y < 0.0 || start_pos.y > game.map_height as f64 {
        return None;
    }
    // // DDA algorithm
    // let ray_start = player_pos;
    // let ray_dir = (player_dir + (cam_plane * 0.0)).normalize();

    
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
    // // let mut out_of_bounds = false;

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
        // match tile {
        //     // It's not solid, so we don't care
        //     0 | 1 => (),
        //     // It's a thin wall! We MIGHT have found a tile
        //     5 | 6 => { 
                
        //     },
        //     // Otherwise it must be solid
        //     _ => tile_found = true,
        // }
        match tile {
            // It's not solid, so we don't care
            0 | 1 => (),
            // It's a thin wall! We MIGHT have found a tile
            6 => {
                // Calculate the intersection point and distance for thin wall going from east to west
                let intersection_x = map_pos.x as f64 + 0.5;  // Assuming thin wall is centered in the block
                let intersection_y = start_pos.y + (intersection_x - start_pos.x) * (dir.y / dir.x);
                let intersection_point = Vector2::new(intersection_x, intersection_y);
                let distance = (intersection_point - start_pos).norm();
                
                if distance < max_dist {
                    return Some((map_pos.y as usize * game.map_width + map_pos.x as usize, intersection_point, distance, RaycastSide::X));
                }
            },
            5 => {
                // Calculate the intersection point and distance for thin wall going from north to south
                let intersection_y = map_pos.y as f64 + 0.5;  // Assuming thin wall is centered in the block
                let intersection_x = start_pos.x + (intersection_y - start_pos.y) * (dir.x / dir.y);
                let intersection_point = Vector2::new(intersection_x, intersection_y);
                let distance = (intersection_point - start_pos).norm();
                
                if distance < max_dist {
                    return Some((map_pos.y as usize * game.map_width + map_pos.x as usize, intersection_point, distance, RaycastSide::Y));
                }
            },
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

