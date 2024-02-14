use std::ops::Add;

use nalgebra::{distance, point, Point2, Vector2};
use rand::{thread_rng, Rng};

use crate::game::{map::{Cell, DoorState}, Game};

type HitPoint = Option<(Point2<f64>, f64, u8)>;

// (distance, texture_along, brightness)
pub fn calc_shape_hit_info(game: &Game, tile_index: usize, ray_gradient: Vector2<f64>, map_pos: Vector2<usize>, start_pos: Vector2<f64>, cell: &Cell) -> Option<(f64, f64, u8)> {

    // The ray will check from it's position to anywhere in front of it.
    // let ray_y_intercept = start_pos.y - ray_gradient * start_pos.x;
    // return None;
    let h = shape_hit(game, cell, tile_index, map_pos, start_pos, ray_gradient);
    if let Some((p, a, b)) = h {
        return Some((p, a, b));
    }
    /*
    // (distance, texture_along, brightness)
    let mut hits: Vec<(Point2<f64>, f64, u8)> = Vec::with_capacity(1);

    match cell.kind {
        // Door
        3 => {
            let amount = match *game.map_m.doors.get(&tile_index).unwrap() {
                DoorState::Closed   => { 1.0 }
                DoorState::Open(..) => { 0.0 }
                DoorState::Closing(a) => { 1.0 - a*2.0 }
                DoorState::Opening(a) => { a*2.0 }
            };

            let pos_s = match cell.flags & 0b00000011 {
                00 => (Vector2::new(map_pos.x as f64 + 0.4,    map_pos.y as f64),
                       Vector2::new(map_pos.x as f64 + 0.6,    map_pos.y as f64 + amount)),
                _  => (Vector2::new(map_pos.x as f64 + 1.0,            map_pos.y as f64 + 0.4),
                       Vector2::new(map_pos.x as f64 + (1.0 - amount), map_pos.y as f64 + 0.6)),
            };
            let mut h = quad_hit(ray_gradient, ray_y_intercept, pos_s.0, pos_s.1);
            hits.append(&mut h);
        }
        // Thick wall
        5 => {
            let pos_s = match cell.flags & 0b00000001 == 0 {
                true =>  (Vector2::new(map_pos.x as f64 + 0.4, map_pos.y as f64),
                          Vector2::new(map_pos.x as f64 + 0.6, map_pos.y as f64 + 1.0)),
                false => (Vector2::new(map_pos.x as f64,       map_pos.y as f64 + 0.4),
                          Vector2::new(map_pos.x as f64 + 1.0, map_pos.y as f64 + 0.6)),
            };
            let mut h = quad_hit(ray_gradient, ray_y_intercept, pos_s.0, pos_s.1);
            hits.append(&mut h);
        }
        // Square Pillar
        6 => {
            let mut h = quad_hit(ray_gradient, ray_y_intercept,
                Vector2::new(map_pos.x as f64 + 0.25, map_pos.y as f64 + 0.25),
                Vector2::new(map_pos.x as f64 + 0.75, map_pos.y as f64 + 0.75));
            hits.append(&mut h);
        }
        // Round pillar
        7 => {
            let mut h = quad_hit(ray_gradient, ray_y_intercept,
                Vector2::new(map_pos.x as f64 + 0.1, map_pos.y as f64 + 0.1),
                Vector2::new(map_pos.x as f64 + 0.9, map_pos.y as f64 + 0.9));
            hits.append(&mut h);
        }
        // Diagonal
        8 => {
            let pos_s = match cell.flags & 0b00000001 == 0 {
                true  => (Vector2::new(map_pos.x as f64, map_pos.y as f64), Vector2::new(map_pos.x as f64 + 1.0, map_pos.y as f64 + 1.0)),
                false => (Vector2::new(map_pos.x as f64 + 1.0, map_pos.y as f64), Vector2::new(map_pos.x as f64, map_pos.y as f64 + 1.0)),
            };
            if let Some(mut h) = line_hit(ray_gradient, ray_y_intercept, pos_s.0, pos_s.1) {
                h.2 = 224;
                hits.push(h);
            }
        }
        _ => return None
    };

    if hits.is_empty() { return None; }
    let start_p = na::point![start_pos.x, start_pos.y];

    // If there's one hit, return that
    if hits.len() == 1 {
        return Some((na::distance(&start_p, &hits[0].0), hits[0].1, hits[0].2));
    }
    // If there are multiple, return the closest one
    else {
        let mut closest_hit = (f64::MAX, 0);
        for (i, hit) in hits.iter().enumerate() {
            let dist = na::distance(&start_p, &hit.0);
            if dist < closest_hit.0 {
                closest_hit = (dist, i);
            }
        }
        return Some((na::distance(&start_p, &hits[closest_hit.1].0), hits[closest_hit.1].1, hits[closest_hit.1].2));
    }*/
    None
}

pub fn shape_hit(game: &Game, cell: &Cell, tile_index: usize, map_pos: Vector2<usize>, ray_pos: Vector2<f64>, ray_dir: Vector2<f64>) -> Option<(f64, f64, u8)>{
    let map_pos_f = point![map_pos.x as f64, map_pos.y as f64];
    // STILL need to make everything use points instead of Vector2.. so this will do for now
    let local_ray_pos = point![ray_pos.x - map_pos_f.x, ray_pos.y - map_pos_f.y];
    // Precalculated as it's probably a teeny bit faster that way
    let ray_grad = ray_dir.y / ray_dir.x;

    let mut hits: Vec<HitPoint> = Vec::with_capacity(1);

    match cell.kind {
        3 => {
            let amount = match *game.map_m.doors.get(&tile_index).unwrap() {
                DoorState::Closed   => { 1.0 }
                DoorState::Open(..) => { 0.0 }
                DoorState::Closing(a) => { 1.0 - a*2.0 }
                DoorState::Opening(a) => { a*2.0 }
            };
            
            hits.push(line_hit_2(local_ray_pos, ray_dir, ray_grad, [point![amount-1.0, 0.6], point![amount, 0.6]], map_pos_f));
            hits.push(line_hit_2(local_ray_pos, ray_dir, ray_grad, [point![amount-1.0, 0.4], point![amount, 0.4]], map_pos_f));
            if let Some(mut edge) = y_line(local_ray_pos, ray_dir, ray_grad, amount, [0.4, 0.6], map_pos_f) {
                edge.1 *= 0.2;
                hits.push(Some(edge));
            }
        }
        _ => { return None }
    }

    let hits: Vec<(Point2<f64>, f64, u8)> = hits.into_iter().flatten().collect();

    if hits.is_empty() { return None; }
    let start_p = na::point![ray_pos.x, ray_pos.y];

    // If there's one hit, return that
    if hits.len() == 1 {
        return Some((na::distance(&start_p, &hits[0].0), hits[0].1, hits[0].2));
    }
    // If there are multiple, return the closest one
    else {
        let mut closest_hit = (f64::MAX, 0);
        for (i, hit) in hits.iter().enumerate() {
            let dist = na::distance(&start_p, &hit.0);
            if dist < closest_hit.0 {
                closest_hit = (dist, i);
            }
        }
        return Some((na::distance(&start_p, &hits[closest_hit.1].0), hits[closest_hit.1].1, hits[closest_hit.1].2));
    }
}



// A line on the X axis.
fn x_line(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, y_intercept: f64, line_bounds: [f64; 2], map_pos: Point2<f64>) -> HitPoint {
    // If the y intercept lies outside the the cell, we don't want it!!
    if !between_in_cell(y_intercept, 0.0, 1.0) { return None; }
    
    let ray_y_intercept = ray_pos.y - ray_grad*ray_pos.x;
    // Derived from substitution
    let x_intercept = (-ray_y_intercept + y_intercept) / ray_grad;
    // If the x intercept lies outside the points or the cell, we don't want it!!
    if !between_in_cell(x_intercept, line_bounds[0], line_bounds[1]) { return None; }
        
    // If the position we found is behind the ray.. we don't want it!!!!!!
    if point_behind_ray(ray_pos, ray_dir, x_intercept, y_intercept) { return None; }

    let along = (x_intercept - f64::min(line_bounds[0], line_bounds[1])) / f64::abs(line_bounds[1] - line_bounds[0]);
    Some((point![map_pos.x + x_intercept, map_pos.y + y_intercept], along, 255))
}

// A line on the Y axis.
fn y_line(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, x_intercept: f64, line_bounds: [f64; 2], map_pos: Point2<f64>) -> HitPoint {
    // If the x intercept lies outside the the cell, we don't want it!!
    if !between_in_cell(x_intercept, 0.0, 1.0) { return None; }
    
    let ray_y_intercept = ray_pos.y - ray_grad*ray_pos.x;
    // Derived from substitution
    let y_intercept = x_intercept * ray_grad + ray_y_intercept;
    // If the y intercept lies outside the points or the cell, we don't want it!!
    if !between_in_cell(y_intercept, line_bounds[0], line_bounds[1]) { return None; }
    
    // If the position we found is behind the ray.. we don't want it!!!!!!
    if point_behind_ray(ray_pos, ray_dir, x_intercept, y_intercept) { return None; }

    let along = (y_intercept - f64::min(line_bounds[0], line_bounds[1])) / f64::abs(line_bounds[1] - line_bounds[0]);
    Some((point![map_pos.x + x_intercept, map_pos.y + y_intercept], along, 192))
}

// Returns if/where the ray hit a given line. (as well as how far along :3)
// from 0 - 1 inside a cell.
fn line_hit_2(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, line_points: [Point2<f64>; 2], map_pos: Point2<f64>) -> HitPoint {
    // If the line is straight along the x axis or y axis, check it the quick (and less error-prone) way.
    if line_points[0].x == line_points[1].x {
        return y_line(ray_pos, ray_dir, ray_grad, line_points[0].x, [line_points[0].y, line_points[1].y], map_pos);
    } else if line_points[0].y == line_points[1].y {
        return x_line(ray_pos, ray_dir, ray_grad, line_points[0].y, [line_points[0].x, line_points[1].x], map_pos);
    }

    let line_grad = (line_points[1].y - line_points[0].y) / (line_points[1].x - line_points[0].x);

    let ray_y_intercept = ray_pos.y - ray_grad*ray_pos.x;

    // Work out where the lines meet on the x axis
    // Derived from substituting the ray's equation (in the form of y=mx+c) into the line's equation (in the form y-y1 = m(x-x1))
    let x_intercept = (((ray_y_intercept - line_points[0].y) / line_grad) + line_points[0].x) / (1.0 - ray_grad/line_grad);
    // If the x intercept lies outside the points or the cell, we don't want it!!
    if !between_in_cell(x_intercept, line_points[0].x, line_points[1].x) { return None; }

    // Calculate by subbing into the ray's equation
    let y_intercept = ray_grad * x_intercept + ray_y_intercept;
    // If the y intercept lies outside the points or the cell, we don't want it!!
    if !between_in_cell(y_intercept, line_points[0].y, line_points[1].y) { return None; }

    // If the position we found is behind the ray.. we don't want it!!!!!!
    // This can only happen when the ray originally starts from inside this shape, so later i might want to add a check for that
    // to avoid unnecessary calculation 
    if point_behind_ray(ray_pos, ray_dir, x_intercept, y_intercept) { return None; }

    // I might be able to calculate 'along' faster by just checking the distance travelled on either the y or x axes,
    // however I'd have to check for edge-cases like the line having 0 change in x or 0 change in y, and I HATE edge cases!!! >:c
    // so this works :3
    let along = distance(&point![x_intercept, y_intercept], &line_points[0]) / distance(&line_points[0], &line_points[1]);
    // TODO: work out brightness
    Some((point![map_pos.x + x_intercept, map_pos.y + y_intercept], along, 255))
}

// Checks if the input is between two values, as well as making sure it's between 0.0 and 1.0
fn between_in_cell(input: f64, p1: f64, p2: f64) -> bool {
    input >= f64::min(p1, p2).clamp(0.0, 1.0) &&
    input <= f64::max(p1, p2).clamp(0.0, 1.0)
}

// Returns true if the point is behind the ray.
fn point_behind_ray(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, x_intercept: f64, y_intercept: f64) -> bool {
    ((ray_dir.x.is_sign_positive() && x_intercept < ray_pos.x) || (ray_dir.x.is_sign_negative() && x_intercept > ray_pos.x)) && 
    ((ray_dir.y.is_sign_positive() && y_intercept < ray_pos.y) || (ray_dir.y.is_sign_negative() && y_intercept > ray_pos.y))
}
// Takes in the ray's gradient and y intercept as well as the lines starting and ending position, then returns where/if it hit.
// Also returns the ratio between the hitpoint and the start, useful for textures!!
fn line_hit(ray_gradient: f64, ray_y_intercept: f64, line_start: Vector2<f64>, line_end: Vector2<f64>) -> Option<(Point2<f64>, f64, u8)> {
    // TODO: Check the direction of the ray to make sure we don't always get a distance if inside the cell
    // Only needed if we're inside the cell, so we could probably save some performance by not checking unless inside

    // Make sure we don't get any wacky things by having an infinite gradient
    // TODO: Think about this and make it better
    let mut line_start = line_start;
    if line_start.y == line_end.y {
        line_start = Vector2::new(line_start.x, line_start.y + 0.000001);
    }

    let line_gradient = (line_end.y - line_start.y) / (line_end.x - line_start.x);

    // See where the two lines meet
    // Derived from substituting the ray's line equation into y - y1 = m(x - x1)
    // This equation doesn't care if you put in line_start or line_end, as it's derived from y - y1 = m(x - x1)
    let x_intercept = (((ray_y_intercept - line_start.y) / line_gradient) + line_start.x) / (1.0 - (ray_gradient/line_gradient));

    // Bounds checking
    // I use min and max in case some NUMPTY put any of the points in line_start after line_end (or vice versa)
    if  x_intercept < line_start.x.min(line_end.x) ||
        x_intercept > line_start.x.max(line_end.x) {
        return None;
    }
    // Calculate y intercept by substituting the x interception into the ray line equation
    let y_intercept = ray_gradient * x_intercept + ray_y_intercept;
    // More bounds checking
    if  y_intercept < line_start.y.min(line_end.y) ||
        y_intercept > line_start.y.max(line_end.y) {
        return None;
    }

    // Need to check if it's inside the cell
    // if x_intercept > 1.0 || x_intercept < 0.0 || y_intercept > 1.0 || y_intercept < 0.0 {
    //     return None;
    // }

    Some((Point2::new(x_intercept, y_intercept), ((line_end.y - y_intercept) / (line_end.y - line_start.y)), 255))
}

fn quad_hit(ray_gradient: f64, ray_y_intercept: f64, quad_start: Vector2<f64>, quad_end: Vector2<f64>) -> Vec<(Point2<f64>, f64, u8)> {
    let mut hits = Vec::new();
    
    if let Some(mut hit) = line_hit(ray_gradient, ray_y_intercept,
        Vector2::new(quad_start.x, quad_start.y),
        Vector2::new(quad_start.x, quad_end.y)) {
            hit.2 = 191;
            hits.push(hit);
    }
    if let Some(hit) = line_hit(ray_gradient, ray_y_intercept,
        Vector2::new(quad_start.x, quad_start.y),
        Vector2::new(quad_end.x, quad_start.y)) {
            hits.push(hit);
    }
    if let Some(hit) = line_hit(ray_gradient, ray_y_intercept,
        Vector2::new(quad_end.x, quad_end.y),
        Vector2::new(quad_start.x, quad_end.y)) {
            hits.push(hit);
    }
    if let Some(mut hit) = line_hit(ray_gradient, ray_y_intercept,
        Vector2::new(quad_end.x, quad_end.y),
        Vector2::new(quad_end.x, quad_start.y)) {
            hit.2 = 191;
            hits.push(hit);
    }
    hits
}

// Todo, make it so i can pass a list of points and it connects them 0 -> 1, 1 -> 2, 2 -> 3, 3 -> 0
// fn ring_hit()

// Circle hit
fn cirle_hit(ray_gradient: f64, ray_y_intercept: f64, center: Vector2<f64>, radius: f64) -> Vec<(Point2<f64>, f64, u8)> {
    let mut hit_points = Vec::with_capacity(2); 
    
    hit_points
}