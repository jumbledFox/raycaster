use std::f32::consts::PI;

use nalgebra::{point, vector, Vector2};

use crate::game::collision;
use super::map::Map;

pub const PLAYER_RADIUS: f32 = 0.3; 

pub struct Player {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub dir: Vector2<f32>,
    pub pitch: f32,
    pub head_bob_amount: f32,
    pub cam_plane: Vector2<f32>,

    pub mid_ray_dist: f32,
}

impl Player {
    pub fn new(pos: Vector2<f32>) -> Player {
        Player {
            pos, dir: Vector2::new(1.0, 0.0), pitch: 0.0,
            vel: Vector2::zeros(), cam_plane: Vector2::new(-1.0, 0.0), head_bob_amount: 0.0, mid_ray_dist: 0.0,
        }
    }

    pub fn step(&mut self, map: &Map, dir: Vector2<f32>, delta: f32) {
        // TODO: better movement
        let dir = dir * 0.6;
        self.vel *= 1.0-(delta * 10.0).min(1.0);
        // I don't like leaving it to decay forever, this works :3
        if self.vel.magnitude() < 0.005 { self.vel = Vector2::zeros() };
        self.vel += na::Rotation2::new(PI / 2.0) * self.dir * dir.x * delta;
        self.vel += self.dir * dir.y * delta;
        
        // self.head_bob_amount = (self.head_bob_amount + self.vel.magnitude().min(1.0) * delta * 20.0 * 1.4).rem_euclid(PI*2.0);
        self.head_bob_amount += self.vel.magnitude().min(1.0) * delta * 30.0;
        self.head_bob_amount = self.head_bob_amount.rem_euclid(PI * 2.0);
        //self.head_bob_amount += (delta * 10.0).rem_euclid(PI*2.0);

        // Very primitive collision detection
        let mov_delta = na::point![self.vel.x * 10.0 * delta, self.vel.y * 10.0 * delta];

        let mut p = point![self.pos.x, self.pos.y];
        collision::slide_mov(&mut p, mov_delta, &map.collision);
        self.pos = vector![p.x, p.y];
        // if map.get(map.coord_to_index(&(newpos.x.floor() as usize), &(self.pos.y.floor() as usize))).kind == 1 {
        // } else {
        //     self.pos.x = newpos.x;
        // }
        // if map.get(map.coord_to_index(&(self.pos.x.floor() as usize), &(newpos.y.floor() as usize))).kind == 1 {
        // } else {
        //     self.pos.y = newpos.y;
        // }

        // self.pos.x += self.vel.x * 10.0 * delta;
        // self.pos.y += self.vel.y * 10.0 * delta;
    }
}