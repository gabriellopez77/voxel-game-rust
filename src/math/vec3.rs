use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> where T : Clone, T: Default {
    pub fn zero() -> Vec3<T> {
        return Vec3 {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }

    pub fn from1f(value: T) -> Vec3<T> {
        return Vec3 {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
        }
    }

    pub fn from3f(x: T, y: T, z:T ) -> Vec3<T> {
        return Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl<T> Add for Vec3<T> where T : Add<Output = T> {
    type Output = Vec3<T>; // Defines the return type

    fn add(self, other: Vec3<T>) -> Vec3<T> {
       return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Mul for Vec3<T> where T : Mul<Output = T> {
    type Output = Vec3<T>; // Defines the return type

    fn mul(self, other: Vec3<T>) -> Vec3<T> {
       return Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Div for Vec3<T> where T : Div<Output = T>{
    type Output = Vec3<T>; // Defines the return type

    fn div(self, other: Vec3<T>) -> Vec3<T> {
       return Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> Sub for Vec3<T> where T : Sub<Output = T>{
    type Output = Vec3<T>; // Defines the return type

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
       return Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}