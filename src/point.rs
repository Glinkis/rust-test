use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::string::ToString;

pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Div<Output = T>> Div for Point<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: AddAssign> Point<T> {
    pub fn add(&mut self, point: Point<T>) {
        self.x += point.x;
        self.y += point.y;
        self.z += point.z;
    }
}

impl<T: SubAssign> Point<T> {
    pub fn sub(&mut self, point: Point<T>) {
        self.x -= point.x;
        self.y -= point.y;
        self.z -= point.z;
    }
}
impl<T: MulAssign> Point<T> {
    pub fn mul(&mut self, point: Point<T>) {
        self.x *= point.x;
        self.y *= point.y;
        self.z *= point.z;
    }
}

impl<T: DivAssign> Point<T> {
    pub fn div(&mut self, point: Point<T>) {
        self.x /= point.x;
        self.y /= point.y;
        self.z /= point.z;
    }
}

impl<T: ToString> Point<T> {
    pub fn to_string(&self) -> String {
        let x = self.x.to_string();
        let y = self.y.to_string();
        let z = self.z.to_string();
        format!("{} {} {}", x, y, z)
    }
}
