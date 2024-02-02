use std::{ops::Mul, f64::consts::PI};

use nalgebra::{Vector2, SimdPartialOrd};

pub struct Player {
    pub pos: Vector2<f64>,
    pub vel: Vector2<f64>,
    pub dir: Vector2<f64>,
    pub pitch: f64,
    pub head_bob_amount: f64,
    pub cam_plane: Vector2<f64>,

    pub lineposa: Vector2<f64>,
    pub lineposb: Vector2<f64>,

    pub mid_ray_dist: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector2::new(1.5, 2.5), dir: Vector2::new(1.0, 0.0), pitch: 0.0,
            vel: Vector2::zeros(), cam_plane: Vector2::new(-1.0, 0.0), head_bob_amount: 0.0, mid_ray_dist: 0.0,
            lineposa: Vector2::zeros(), lineposb: Vector2::zeros(), }
    }

    pub fn step(&mut self, dir: Vector2<f64>, delta: f64) {
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
        self.pos.x += self.vel.x * 10.0 * delta;
        self.pos.y += self.vel.y * 10.0 * delta;
    }
}