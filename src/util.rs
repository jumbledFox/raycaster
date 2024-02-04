use std::mem::swap;

use nalgebra::Vector2;

use crate::{WIDTH, game::Game};

use rand::Rng;


type RaycastResult = Option<(
    usize,        // Index of cell it hit
    Vector2<f64>, // Hit point
    f64,          // Distance
    RaycastSide   // Side
)>;

type CellResult = Option<(
    usize,      // Index of cell found

)>;

#[derive(Debug, PartialEq)]
pub enum RaycastSide { X, Y }

// Shoots a raycast from a position and a direction and returns what it hit (as an index in the map),
// the hit point, how far away it was, and if it hit x or y!
pub fn raycast(game: &Game, start_pos: Vector2<f64>, dir: Vector2<f64>, max_dist: f64, tell_info: bool) -> Option<(usize, Vector2<f64>, f64, RaycastSide)> {
    // If the ray is out of bounds, don't bother
    if  start_pos.x < 0.0 || start_pos.x > game.map_width  as f64 ||
        start_pos.y < 0.0 || start_pos.y > game.map_height as f64 {
        return None;
    }

    let dir = dir.normalize();

    // Which cell of the map we're in
    let mut map_pos: Vector2<usize> = Vector2::new(
        start_pos.x as usize,
        start_pos.y as usize);
    // Which cell of the map we WILL be in next time
    let mut next_map_pos = map_pos;
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

    let distance = 0.0f64;

    // let next_pos = match ray_length_1d.x < ray_length_1d.y {
    //     true  => {
    //         map_pos.x = map_pos.x.saturating_add_signed(step_x);
    //         ray_length_1d.x + step_size.x
    //     }
    //     false => {
    //         map_pos.y = map_pos.y.saturating_add_signed(step_y);
    //         ray_length_1d.y + step_size.y
    //     }
    // };

    let mut current_pos = start_pos;
    let mut next_pos;
    // Move next_pos along the shortest axis
    if ray_length_1d.x < ray_length_1d.y {
        next_map_pos.x = next_map_pos.x.saturating_add_signed(step_x);
        ray_length_1d.x += step_size.x;
        next_pos = start_pos + dir * ray_length_1d.x;
    } else {
        next_map_pos.y = next_map_pos.y.saturating_add_signed(step_y); 
        ray_length_1d.x += step_size.y;
        next_pos = start_pos + dir * ray_length_1d.y;
    }

    // Check cell the player starts in
    if let Some(..) = check_cell(game, *game.map.get(game.coord_to_index(&(map_pos.x, map_pos.y))).unwrap(), current_pos, next_pos) {
        return Some((game.coord_to_index(&(map_pos.x, map_pos.y)), Vector2::zeros(), 0.01, RaycastSide::X));
    }

    swap(&mut current_pos, &mut next_pos);
    swap(&mut map_pos, &mut next_map_pos);
    
    // Ok, the ray didn't hit anything in the cell we're currently in, let's do DDA
    let mut distance: f64 = 0.0;
    // let mut side = RaycastSide::X;
    
    let mut tile_found = false;
    while !tile_found && distance < max_dist {
        // Move next_pos along the shortest axis
        if ray_length_1d.x < ray_length_1d.y {
            distance = ray_length_1d.x;

            next_map_pos.x = next_map_pos.x.saturating_add_signed(step_x);
            ray_length_1d.x += step_size.x;
            next_pos = start_pos + dir * ray_length_1d.x;
        } else {
            distance = ray_length_1d.y;

            next_map_pos.y = next_map_pos.y.saturating_add_signed(step_y); 
            ray_length_1d.x += step_size.y;
            next_pos = start_pos + dir * ray_length_1d.y;
        }
        if let Some(c) = check_cell(game, *game.map.get(game.coord_to_index(&(map_pos.x, map_pos.y))).unwrap(), current_pos, next_pos) {
            return Some((game.coord_to_index(&(map_pos.x, map_pos.y)), Vector2::zeros(), distance+c, RaycastSide::X));
        }
        swap(&mut current_pos, &mut next_pos);
        swap(&mut map_pos, &mut next_map_pos);
    }
    // For correcting bulge, instead of this method, which doesn't seem to work:
    // https://lodev.org/cgtutor/raycasting.html
    // i multiply the distance by cos of the angle, as shown here:
    // https://www.permadi.com/tutorial/raycast/rayc8.html

    // match tile_found && distance < max_dist {
    //     true =>  {
    //         let perp_dist = distance*dir.angle(&game.player.dir).cos();
    //         Some((map_pos.y as usize * game.map_width + map_pos.x as usize, start_pos + dir*perp_dist, perp_dist, side))
    //     }
    //     false => None
    // }
    None
}

// Checks if a ray collided with the cell
// Returns the distance of the collision from ray_start, as well as maybe how bright it should be or something.
fn check_cell(game: &Game, cell: u8, ray_start: Vector2<f64>, ray_end: Vector2<f64>) -> Option<f64> {
    match cell {
        // Not solid
        0 | 1 => None,
        // Thin wall, E/W
        6 => {
            None
        },
        // Completely solid
        _ => Some(0.00000001),
    }
}
