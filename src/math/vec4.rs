use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Vec4<T> where T : Clone, T: Default {
    pub fn zero() -> Vec4<T> {
        return Vec4 {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default()
        }
    }

    pub fn from1f(value: T) -> Vec4<T> {
        return Vec4 {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
            w: value.clone()
        }
    }

    pub fn from4f(x: T, y: T, z: T, w: T) -> Vec4<T> {
        return Vec4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
}

impl<T> Add for Vec4<T> where T : Add<Output = T> {
    type Output = Vec4<T>; // Defines the return type

    fn add(self, other: Vec4<T>) -> Vec4<T> {
       return Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T> Mul for Vec4<T> where T : Mul<Output = T> {
    type Output = Vec4<T>; // Defines the return type

    fn mul(self, other: Vec4<T>) -> Vec4<T> {
       return Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T> Div for Vec4<T> where T : Div<Output = T>{
    type Output = Vec4<T>; // Defines the return type

    fn div(self, other: Vec4<T>) -> Vec4<T> {
       return Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T> Sub for Vec4<T> where T : Sub<Output = T>{
    type Output = Vec4<T>; // Defines the return type

    fn sub(self, other: Vec4<T>) -> Vec4<T> {
       return Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}