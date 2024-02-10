use nalgebra::{Point2, Vector2};
use rand::Rng;

use crate::game::map::Cell;

// (distance, texture_along, brightness)
pub fn calc_shape_hit_info(ray_gradient: f64, map_pos: Vector2<usize>, start_pos: Vector2<f64>, cell: Cell) -> Option<(f64, f64, u8)> {
    // (distance, texture_along, brightness)
    let mut hits: Vec<(Point2<f64>, f64, u8)> = Vec::with_capacity(1);

    match cell.kind {
        5 => {
            let diagonal_y_intercept = map_pos.y as f64 - map_pos.x as f64;

            let x_intersection = (start_pos.y - (ray_gradient*start_pos.x) - diagonal_y_intercept) / (1.0 - ray_gradient);
            
            // If the point of intersection isn't in the cell, we don't wanna render it!
            if !(x_intersection > map_pos.x as f64 + 1.0 || x_intersection < map_pos.x as f64) {
                let y_intersection = x_intersection + diagonal_y_intercept;
                hits.push((Point2::new(x_intersection, y_intersection), 0.7, 255));
            }
        }
        5 => {
            // derived from line equations
            let x_intersection = (0.6 + map_pos.y as f64 - start_pos.y + (ray_gradient * start_pos.x)) / ray_gradient;
            
            // If the point of intersection isn't in the cell, we don't wanna render it!
            if !(x_intersection > map_pos.x as f64 + 1.0 || x_intersection < map_pos.x as f64) {
                let y_intersection = 0.6 + map_pos.y as f64;
                hits.push((Point2::new(x_intersection, y_intersection), 0.7, 255));
            }

            // derived from line equations
            let x_intersection = (0.4 + map_pos.y as f64 - start_pos.y + (ray_gradient * start_pos.x)) / ray_gradient;
                        
            // If the point of intersection isn't in the cell, we don't wanna render it!
            if !(x_intersection > map_pos.x as f64 + 1.0 || x_intersection < map_pos.x as f64) {
                let y_intersection = 0.4 + map_pos.y as f64;
                hits.push((Point2::new(x_intersection, y_intersection), 0.7, 255));
            }
        }
        _ => return None,
    };

    if hits.is_empty() { return None; }
    let start_p = na::point![start_pos.x, start_pos.y];
    // If there's one hit, return that
    if hits.len() == 1 {
        return Some((na::distance(&start_p, &hits[0].0), 0.5, 255));
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
        return Some((na::distance(&start_p, &hits[closest_hit.1].0), 0.5, 255));
    }
}