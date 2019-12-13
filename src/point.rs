pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn add(&mut self, point: &Point) {
        self.x += point.x;
        self.y += point.y;
        self.z += point.z;
    }
    pub fn sub(&mut self, point: &Point) {
        self.x -= point.x;
        self.y -= point.y;
        self.z -= point.z;
    }
    pub fn mul(&mut self, point: &Point) {
        self.x *= point.x;
        self.y *= point.y;
        self.z *= point.z;
    }
    pub fn div(&mut self, point: &Point) {
        self.x /= point.x;
        self.y /= point.y;
        self.z /= point.z;
    }
}
