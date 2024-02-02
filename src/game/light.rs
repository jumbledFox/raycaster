use nalgebra::Vector2;

// Todo: Colour ?
pub struct Light {
    pub pos: Vector2<f64>,
}

impl Light {
    pub fn new(pos: Vector2<f64>) -> Light {
        Light { pos }
    }
}