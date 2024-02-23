use lerp::num_traits::float;
use nalgebra::{point, Point1, Point2};

use crate::game::collision;

use super::player::PLAYER_RADIUS;

pub type Segment = [Point2<f32>; 2];

// http://code.alaiwan.org/blog/collision-disk.html

pub struct Collision {
    pub depth: f32,
    pub normal: Point2<f32>,
}

pub fn slide_mov(pos: &mut Point2<f32>, delta: Point2<f32>, segments: &Vec<Segment>) {
    pos.x += delta.x;
    pos.y += delta.y;

    for _ in 0..7 {
        if let Some(collision) = collide_with_segments(*pos, segments) {
            if collision.depth == 0.0 { break; }
            pos.x += collision.normal.x * collision.depth;
            pos.y += collision.normal.y * collision.depth;
        } else {
            break;
        }
    }
}

pub fn collide_with_segments(pos: Point2<f32>, segments: &Vec<Segment>) -> Option<Collision> {
    let mut deepest: Option<Collision> = None;
    for seg in segments {
        if let Some(collision) = collide_disk_with_segment(pos, *seg) {
            deepest = match &deepest {
                None => Some(collision),
                Some(d) => {
                    if collision.depth > d.depth {
                        Some(collision)
                    } else {
                        deepest
                    }
                }
            };
        }
        
    }
   deepest
}

pub fn collide_disk_with_segment(disk_center: Point2<f32>, seg: Segment) -> Option<Collision> {
    let delta = disk_center - closest_point_on_seg(disk_center, seg);

    if point_2_cmp_mul(delta.into(), delta.into()) > PLAYER_RADIUS * PLAYER_RADIUS { return None; }

    let dist = delta.magnitude();
    let n = delta * (1.0 / dist);
    Some(Collision { depth: PLAYER_RADIUS - dist, normal: n.into() })
}

// Returns the point from 'seg' which is closest to 'p'
pub fn closest_point_on_seg(p: Point2<f32>, seg: Segment) -> Point2<f32> {
    let tangent = seg[1] - seg[0];
    
    if (p - seg[0]).dot(&tangent) <= 0.0 {
        return seg[0];
    }
    if (p - seg[1]).dot(&tangent) >= 0.0 {
        return seg[1];
    }
    // return seg[0];
    let t = tangent.normalize();
    let relative_pos = p - seg[0];
    return seg[0] + t * point_2_cmp_mul(t.into(), relative_pos.into());
}

pub fn point_2_cmp_mul(a: Point2<f32>, b: Point2<f32>) -> f32 {
    a.x * b.x + a.y * b.y
}