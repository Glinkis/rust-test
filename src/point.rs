use std::ops::{Add, Div, Mul, Sub};

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Div for Point {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
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
