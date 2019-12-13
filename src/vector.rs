pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn add(&mut self, vector: &Vector2) {
        self.x += vector.x;
        self.y += vector.y;
    }
    pub fn sub(&mut self, vector: &Vector2) {
        self.x -= vector.x;
        self.y -= vector.y;
    }
    pub fn mul(&mut self, vector: &Vector2) {
        self.x *= vector.x;
        self.y *= vector.y;
    }
    pub fn div(&mut self, vector: &Vector2) {
        self.x /= vector.x;
        self.y /= vector.y;
    }
}
