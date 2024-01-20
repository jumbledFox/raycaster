use nalgebra::Vector2;

use crate::{WIDTH, game::Game};

#[derive(PartialEq)]
pub enum RaycastSide { X, Y }

// Shoots a raycast from a position and a direction and returns what it hit (as an index in the map), how far away it was, and if it hit x or y!
pub fn raycast(game: &Game, start_pos: Vector2<f64>, dir: Vector2<f64>, max_dist: f64) -> Option<(usize, f64, RaycastSide)> {
    
    // // DDA algorithm
    // let ray_start = player_pos;
    // let ray_dir = (player_dir + (cam_plane * 0.0)).normalize();

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

        if  start_pos.x < 0.0 || start_pos.x.ceil() >= game.map_width  as f64 || 
            start_pos.y < 0.0 || start_pos.y.ceil() >= game.map_height as f64 {
            continue;
        }


        // check_points.push(ray_start + (ray_dir * distance));


        // let map_x = ray.pos.x.floor() as usize;
        // let map_y = ray.pos.y.floor() as (Hello) usize;
        // TODO: bounds checking and tidy up
        // if map_pos.x > MAP_WIDTH+1 || map_pos.y > MAP_HEIGHT+1 {
        //     continue;
        // }

        if  map_pos.x > game.map_width as isize - 1  || map_pos.y > game.map_height as isize - 1 ||
            map_pos.x < 0 || map_pos.y < 0 {
            continue;
        }
        // if map[map_pos.x + map_pos.y * MAP_WIDTH] != 0 {
        //     //println!("dist: {:?}", f64::sqrt((ray_pos[0]-player_pos[0]).powi(2)+(ray_pos[1]-player_pos[1]).powi(2)));
        //     //hit_pos = Some(ray.pos);
        //     tile_found = true;
        // }
        if game.map[map_pos.x as usize + map_pos.y as usize * game.map_width] != 0 {
            //println!("dist: {:?}", f64::sqrt((start_pos[0]-player_pos[0]).powi(2)+(ray_pos[1]-player_pos[1]).powi(2)));
            //hit_pos = Some(ray.pos);
            tile_found = true;
        }
    }
    
    // //hit_pos = None;
    match tile_found {
        //true =>  Some(start_pos + (dir * distance)),
        true =>  Some((map_pos.y as usize * game.map_width + map_pos.x as usize, distance, side)),
        false => None
    }
}

