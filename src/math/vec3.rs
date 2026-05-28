use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};

use crate::math::{Vec3i, Vec4};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const UP: Vec3 = Vec3{ x: 0.0, y: 1.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32 ) -> Self { Self { x, y, z, } }
    pub fn from1(value: f32) -> Self { Self { x: value, y: value, z: value } }
    pub fn from4(v: Vec4) -> Self { Self { x: v.x, y: v.y, z: v.z } }

    pub fn as_ptr(&self) -> *const f32 { &self.x }

    pub fn as_vec3i(&self) -> Vec3i {
        Vec3i {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }


    pub fn dot(&self, other: Self) -> f32 {
        let temp = *self * other;

        temp.x + temp.y + temp.z
    }

    pub fn normalized(&self) -> Self {
        let temp = *self;

        temp * 1.0 / temp.dot(temp).sqrt()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn length(&self) -> f32 { (self.x * self.x) + (self.y * self.y) + (self.z * self.z).sqrt() }
}


impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool { self.x == other.x && self.y == other.y && self.z == other.z }
}

impl Add for Vec3 { type Output = Self; fn add(self, o: Self) -> Self { Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z) } }
impl Mul for Vec3 { type Output = Self; fn mul(self, o: Self) -> Self { Vec3::new(self.x * o.x, self.y * o.y, self.z * o.z) } }
impl Div for Vec3 { type Output = Self; fn div(self, o: Self) -> Self { Vec3::new(self.x / o.x, self.y / o.y, self.z / o.z) } }
impl Sub for Vec3 { type Output = Self; fn sub(self, o: Self) -> Self { Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z) } }
impl Add<f32> for Vec3 { type Output = Self; fn add(self, o: f32) -> Self { Vec3::new(self.x + o, self.y + o, self.z + o) } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, o: f32) -> Self { Vec3::new(self.x * o, self.y * o, self.z * o) } }
impl Div<f32> for Vec3 { type Output = Self; fn div(self, o: f32) -> Self { Vec3::new(self.x / o, self.y / o, self.z / o) } }
impl Sub<f32> for Vec3 { type Output = Self; fn sub(self, o: f32) -> Self { Vec3::new(self.x - o, self.y - o, self.z - o) } }

impl AddAssign for Vec3 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; self.z += o.z; } }
impl MulAssign for Vec3 { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; self.z *= o.z; } }
impl DivAssign for Vec3 { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; self.z /= o.z; } }
impl SubAssign for Vec3 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; self.z -= o.z; } }
impl AddAssign<f32> for Vec3 { fn add_assign(&mut self, o: f32) { self.x += o; self.y += o; self.z += o; } }
impl MulAssign<f32> for Vec3 { fn mul_assign(&mut self, o: f32) { self.x *= o; self.y *= o; self.z *= o; } }
impl DivAssign<f32> for Vec3 { fn div_assign(&mut self, o: f32) { self.x /= o; self.y /= o; self.z /= o; } }
impl SubAssign<f32> for Vec3 { fn sub_assign(&mut self, o: f32) { self.x -= o; self.y -= o; self.z -= o; } }
