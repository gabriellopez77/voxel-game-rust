use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> where T : Clone, T: Default {
    pub fn zero() -> Vec2<T> {
        return Vec2 {
            x: Default::default(),
            y: Default::default(),
        }
    }

    pub fn from1f(value: T) -> Vec2<T> {
        return Vec2 {
            x: value.clone(),
            y: value.clone(),
        }
    }
}

impl<T> Add for Vec2<T> where T : Add<Output = T> {
    type Output = Vec2<T>; // Defines the return type

    fn add(self, other: Vec2<T>) -> Vec2<T> {
       return Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Mul for Vec2<T> where T : Mul<Output = T> {
    type Output = Vec2<T>; // Defines the return type

    fn mul(self, other: Vec2<T>) -> Vec2<T> {
       return Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Div for Vec2<T> where T : Div<Output = T>{
    type Output = Vec2<T>; // Defines the return type

    fn div(self, other: Vec2<T>) -> Vec2<T> {
       return Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T : Sub<Output = T>{
    type Output = Vec2<T>; // Defines the return type

    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}