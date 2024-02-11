use nalgebra::{Vector, Vector2};

use crate::{WIDTH, game::Game};

pub mod shape;
use shape::calc_shape_hit_info;

use rand::Rng;

// Shoots a raycast from a position and a direction and returns what it hit (as an index in the map),
// the hit point, how far away it was, and if it hit x or y!
// (cell, hit_pos, distance, texture_along, side)

// (cell, distance, texture_along, brightness, side)
type RaycastResult = Option<(usize, f64, f64, u8, u8)>;

pub fn raycast(game: &Game, start_pos: Vector2<f64>, dir: Vector2<f64>, max_dist: f64, tell_info: bool) -> RaycastResult {
    // If the ray is out of bounds, don't bother.
    if  start_pos.x < 0.0 || start_pos.x > game.map_width  as f64 ||
        start_pos.y < 0.0 || start_pos.y > game.map_height as f64 {
        return None;
    }

    // Which box of the map we're in
    let mut map_pos: Vector2<usize> = Vector2::new(
        start_pos.x as usize,
        start_pos.y as usize);
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

    // Set initially to a very small value to avoid division by zero in other functions. (Namely render_view())
    let mut distance: f64 = 0.000000000001;

    let mut side = 0;

    loop {
        // If out of bounds, stop checking
        if map_pos.x >= game.map_width || map_pos.y >= game.map_height { break; }

        // If the distance is too large, stop checking
        if distance > max_dist { break; }

        // Get the tile at the current position, check it out
        let tile_index = game.coord_to_index(&map_pos.x, &map_pos.y);
        let tile = game.map.get(tile_index).unwrap();
        let t = game.map_m.get(tile_index);
        match t.kind {
            // Air | Light, obviously don't want to render this
            0 | 2 => {}
            // Solid cube
            1 => {
                // Calculate the perpendicular distance
                // https://www.permadi.com/tutorial/raycast/rayc8.html
                let perp_dist = distance*dir.angle(&game.player.dir).cos();

                let texture_along: f64;
                // TODO: Store sides and use them to determine this :3
                if side == 0 {
                    texture_along = (start_pos + perp_dist * dir).y.rem_euclid(1.0);
                } else {
                    texture_along = (start_pos + perp_dist * dir).x.rem_euclid(1.0);
                }
                
                return Some((tile_index, perp_dist, texture_along, side * 64 + 191, side));
            }
            // Other shape...
            _ => {
                let shape_result = calc_shape_hit_info(game, tile_index, dir.y/dir.x, map_pos, start_pos, game.map_m.get(tile_index));
                if let Some((distance, texture_along, brightness)) = shape_result {
                    let perp_dist = distance*dir.angle(&game.player.dir).cos();
                    return Some((tile_index, perp_dist, texture_along, brightness, side));
                }
            }
        }

        // Move along either X or Y, depending on which ray is shorter
        if ray_length_1d.x < ray_length_1d.y {
            // If map_pos goes below zero, it's obviously not gonna hit anything.
            if let Some(t) = map_pos.x.checked_add_signed(step_x) {
                map_pos.x = t;
            } else { break; }
            // Set the distance to be however long the shortest ray is
            distance = ray_length_1d.x;
            // Step 1 along the X axis to the next intersection
            ray_length_1d.x += step_size.x;
            side = 0;
        } else {
            // If map_pos goes below zero, it's obviously not gonna hit anything.
            if let Some(t) = map_pos.y.checked_add_signed(step_y) {
                map_pos.y = t;
            } else { break; }
            // Set the distance to be however long the shortest ray is
            distance = ray_length_1d.y;
            // Step 1 along the Y axis to the next intersection
            ray_length_1d.y += step_size.y;
            side = 1;
        }
    }

    None
}