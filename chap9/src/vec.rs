#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    pub const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };

    pub fn scaled_by(self, s: f32) -> Vector2 {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}
