pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn add(&mut self, vector: &Point) {
        self.x += vector.x;
        self.y += vector.y;
    }
    pub fn sub(&mut self, vector: &Point) {
        self.x -= vector.x;
        self.y -= vector.y;
    }
    pub fn mul(&mut self, vector: &Point) {
        self.x *= vector.x;
        self.y *= vector.y;
    }
    pub fn div(&mut self, vector: &Point) {
        self.x /= vector.x;
        self.y /= vector.y;
    }
}
