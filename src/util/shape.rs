use std::ops::Add;

use nalgebra::{distance, point, Point2, Vector2};
use rand::{thread_rng, Rng};

use crate::game::{map::{Cell, DoorState}, Game};

type HitPoint = (Point2<f64>, f64, u8);

pub type Segment = [Point2<f64>; 2];

// What i want to do:
// store all shapes in some easy form, like a vector of segments
// I don't want to regenerate them each time so I've got to hard-code the segments and generate their equations
// Shapes are made up of lines (and maybe in the future curves, i could have an enum that has varients like line, x_line, y_line
// (for a line, a line on the x axis, a line on the y axis, etc) then i could have a function that takes in the enum and does stuff

pub enum ShapePart {
    Line(f64, f64, f64, f64),
    LineX(f64, f64, f64),
    LineY(f64, f64, f64),
}
pub fn get_shape_segments(_cell: &Cell) {
    
}

trait SetIfSmaller {
    fn set_if_smaller(&mut self, pos: Point2<f64>, other: Option<HitPoint>);
}

impl SetIfSmaller for Option<HitPoint> {
    fn set_if_smaller(&mut self, pos: Point2<f64>, other: Option<HitPoint>) {
        if other.is_none() { return; }
        if self.is_none() {
            *self = other;
            return;
        }
        let my_dist    = na::distance(&pos, &self .unwrap().0);
        let other_dist = na::distance(&pos, &other.unwrap().0);
        if other_dist < my_dist {
            *self = other;
        }
    }
}

pub fn shape_hit(game: &Game, cell: &Cell, tile_index: usize, map_pos: Vector2<usize>, ray_pos: Vector2<f64>, ray_dir: Vector2<f64>) -> Option<(f64, f64, u8)>{
    let map_pos_f = point![map_pos.x as f64, map_pos.y as f64];
    // STILL need to make everything use points instead of Vector2.. so this will do for now
    let local_ray_pos = point![ray_pos.x - map_pos_f.x, ray_pos.y - map_pos_f.y];
    // Precalculated as it's probably a teeny bit faster that way
    let ray_grad = ray_dir.y / ray_dir.x;

    let mut hit: Option<HitPoint> = None;
    let ray_pos_p = na::point![ray_pos.x, ray_pos.y];

    match cell.kind {
        // DOOR
        3 => {
            let amount = match *game.map.doors.get(&tile_index).unwrap() {
                DoorState::Closed   => { 1.0 }
                DoorState::Open(..) => { 0.0 }
                DoorState::Closing(a) => { 1.0 - a*2.0 }
                DoorState::Opening(a) => { a*2.0 }
            };

            let orientation = cell.flags & 0b00000001 == 0;

            match (cell.flags & 0b00001100) >> 2 {
            // SLIDE DOOR
            0 => {
                // The left and the right of the door
                // Changes depending on the flipped flag
                let door_sides = match (cell.flags & 0b00000010) >> 1 {
                    0 => [amount, amount-1.0],
                    _ => [1.0-amount, 2.0-amount],
                };

                // Two long sides
                hit.set_if_smaller(ray_pos_p, line_axis(orientation, (1.0, 0.0), local_ray_pos, ray_dir, ray_grad, 0.6, door_sides, map_pos_f));
                hit.set_if_smaller(ray_pos_p, line_axis(orientation, (1.0, 0.0), local_ray_pos, ray_dir, ray_grad, 0.4, door_sides, map_pos_f));
                // Short bit
                hit.set_if_smaller(ray_pos_p, line_axis(!orientation, (0.2, 0.4), local_ray_pos, ray_dir, ray_grad, door_sides[0], [0.4, 0.6], map_pos_f));
            },
            // ELEVATOR DOOR
            1 => {
                let door_parts = [[amount / 2.0, (amount - 1.0) / 2.0], [1.0-(amount / 2.0), 1.5-(amount / 2.0)]];
                hit.set_if_smaller(ray_pos_p, line_axis(orientation, (0.5, 0.0), local_ray_pos, ray_dir, ray_grad, 0.55, door_parts[0], map_pos_f));
                hit.set_if_smaller(ray_pos_p, line_axis(orientation, (0.5, 0.0), local_ray_pos, ray_dir, ray_grad, 0.45, door_parts[0], map_pos_f));
                hit.set_if_smaller(ray_pos_p, line_axis(orientation, (0.5, 0.5), local_ray_pos, ray_dir, ray_grad, 0.55, door_parts[1], map_pos_f));
                hit.set_if_smaller(ray_pos_p, line_axis(orientation, (0.5, 0.5), local_ray_pos, ray_dir, ray_grad, 0.45, door_parts[1], map_pos_f));
                // Short bit
                hit.set_if_smaller(ray_pos_p, line_axis(!orientation, (4.0 / 128.0, 62.0 / 128.0), local_ray_pos, ray_dir, ray_grad, door_parts[0][0], [0.45, 0.55], map_pos_f));
                hit.set_if_smaller(ray_pos_p, line_axis(!orientation, (4.0 / 128.0, 62.0 / 128.0), local_ray_pos, ray_dir, ray_grad, door_parts[1][0], [0.45, 0.55], map_pos_f));
            }
            _ => {}
            }
            
        }
        // THIN WALL
        4 => {
            let orientation = cell.flags & 0b00000001 == 0;
            hit.set_if_smaller(ray_pos_p, line_axis(orientation, (1.0, 0.0), local_ray_pos, ray_dir, ray_grad, 0.5, [0.0, 1.0], map_pos_f));
        }
        // THICK WALL
        5 => {
            let orientation = cell.flags & 0b00000001 == 0;
            // Long bits
            hit.set_if_smaller(ray_pos_p, line_axis(orientation, (1.0, 0.0), local_ray_pos, ray_dir, ray_grad, 0.6, [0.0, 1.0], map_pos_f));
            hit.set_if_smaller(ray_pos_p, line_axis(orientation, (1.0, 0.0), local_ray_pos, ray_dir, ray_grad, 0.4, [0.0, 1.0], map_pos_f));
            // Short bits
            // TODO: Make a better way to manipulate the values maybe..
            hit.set_if_smaller(ray_pos_p, line_axis(!orientation, (0.2, 0.4), local_ray_pos, ray_dir, ray_grad, 0.0, [0.4, 0.6], map_pos_f));
            hit.set_if_smaller(ray_pos_p, line_axis(!orientation, (0.2, 0.4), local_ray_pos, ray_dir, ray_grad, 1.0, [0.4, 0.6], map_pos_f));
        }
        // SQUARE PILLAR
        6 => {
            hit.set_if_smaller(ray_pos_p, line_axis(true , (0.5, 0.25), local_ray_pos, ray_dir, ray_grad, 0.25, [0.25, 0.75], map_pos_f));
            hit.set_if_smaller(ray_pos_p, line_axis(true , (0.5, 0.25), local_ray_pos, ray_dir, ray_grad, 0.75, [0.25, 0.75], map_pos_f));
            hit.set_if_smaller(ray_pos_p, line_axis(false, (0.5, 0.25), local_ray_pos, ray_dir, ray_grad, 0.25, [0.25, 0.75], map_pos_f));
            hit.set_if_smaller(ray_pos_p, line_axis(false, (0.5, 0.25), local_ray_pos, ray_dir, ray_grad, 0.75, [0.25, 0.75], map_pos_f));
        }
        // ROUND PILLAR
        7 => {
            return None;
        }
        // DIAGONAL
        8 => {
            let orientation = cell.flags & 0b00000001 == 0;
            let positions = match orientation {
                true  => [point![0.0, 0.0], point![1.0, 1.0]],
                false => [point![1.0, 0.0], point![0.0, 1.0]],
            };
            hit.set_if_smaller(ray_pos_p, line(local_ray_pos, ray_dir, ray_grad, positions, map_pos_f));
        }
        _ => { return None }
    }

    if hit.is_none() { return None; }
    return Some((na::distance(&ray_pos_p, &hit.unwrap().0), hit.unwrap().1, hit.unwrap().2));

}

fn quad(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, intercept: f64, rect_start: Point2<f64>) {

}

// A line on an axis 
// If `axis`` is false the line is on the X axis, otherwise it's on the Y
// tex.0 is the texture stretch, tex.1 is the texture offset.
fn line_axis(axis: bool, tex: (f64, f64), ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, intercept: f64, line_bounds: [f64; 2], map_pos: Point2<f64>)
    -> Option<HitPoint> {
    
    let l = match axis {
        false => line_x(ray_pos, ray_dir, ray_grad, intercept, line_bounds, map_pos),
        true  => line_y(ray_pos, ray_dir, ray_grad, intercept, line_bounds, map_pos),
    };
    if let Some(mut line) = l {
        // TODO: Make textures flip depending on direction viewed
        // if ray_dir.x.is_sign_negative() || ray_dir.y.is_sign_negative() {
        //     line.1 = 1.0-line.1;
        // }
        line.1 *= tex.0;
        line.1 += tex.1;
        Some(line)
    } else { None }
}

// A line on the X axis.
fn line_x(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, y_intercept: f64, line_bounds: [f64; 2], map_pos: Point2<f64>) -> Option<HitPoint> {
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
fn line_y(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, x_intercept: f64, line_bounds: [f64; 2], map_pos: Point2<f64>) -> Option<HitPoint> {
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
fn line(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, ray_grad: f64, line_points: [Point2<f64>; 2], map_pos: Point2<f64>) -> Option<HitPoint> {
    // If the line is straight along the x axis or y axis, check it the quick (and less error-prone) way.
    if line_points[0].x == line_points[1].x {
        return line_y(ray_pos, ray_dir, ray_grad, line_points[0].x, [line_points[0].y, line_points[1].y], map_pos);
    } else if line_points[0].y == line_points[1].y {
        return line_x(ray_pos, ray_dir, ray_grad, line_points[0].y, [line_points[0].x, line_points[1].x], map_pos);
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
    let mut along = distance(&point![x_intercept, y_intercept], &line_points[0]) / distance(&line_points[0], &line_points[1]);
    // TODO: Make textures always face the right way
    // if ray_dir.x.is_sign_negative() || ray_dir.y.is_sign_positive() {
    //     along = 1.0-along;
    // }
    // TODO: work out brightness
    Some((point![map_pos.x + x_intercept, map_pos.y + y_intercept], along, 255))
}

// Checks if the input is between two values, as well as making sure it's between 0.0 and 1.0
fn between_in_cell(input: f64, p1: f64, p2: f64) -> bool {
    input >= f64::min(p1, p2).clamp(0.0, 1.0) &&
    input <= f64::max(p1, p2).clamp(0.0, 1.0)
}

// Returns true if the point is behind the ray.
// TODO ??
fn point_behind_ray(ray_pos: Point2<f64>, ray_dir: Vector2<f64>, x_intercept: f64, y_intercept: f64) -> bool {
    ((ray_dir.x.is_sign_positive() && x_intercept < ray_pos.x) || (ray_dir.x.is_sign_negative() && x_intercept > ray_pos.x)) && 
    ((ray_dir.y.is_sign_positive() && y_intercept < ray_pos.y) || (ray_dir.y.is_sign_negative() && y_intercept > ray_pos.y))
}