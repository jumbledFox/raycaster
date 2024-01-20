use nalgebra::Vector2;


pub struct Player {
    pub pos: Vector2<f64>,
    pub dir: Vector2<f64>,
}

impl Player {
    pub fn new() -> Player {
        Player { pos: Vector2::new(1.0, 1.0), dir: Vector2::new(0.0, 1.0) }
    }
}