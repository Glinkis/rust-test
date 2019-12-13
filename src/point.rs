pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn add(&mut self, point: &Point) {
        self.x += point.x;
        self.y += point.y;
    }
    pub fn sub(&mut self, point: &Point) {
        self.x -= point.x;
        self.y -= point.y;
    }
    pub fn mul(&mut self, point: &Point) {
        self.x *= point.x;
        self.y *= point.y;
    }
    pub fn div(&mut self, point: &Point) {
        self.x /= point.x;
        self.y /= point.y;
    }
}
