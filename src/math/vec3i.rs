use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};
use std::hash::*;

use crate::math::Vec3;


#[repr(C)]
#[derive(Clone, Copy, Eq)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vec3i {
    pub const ZERO: Vec3i = Vec3i { x: 0, y: 0, z: 0 };

    pub fn new(x: i32, y: i32, z: i32 ) -> Self { Self { x, y, z, } }
    pub fn from1(value: i32) -> Self { Self { x: value, y: value, z: value } }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }
}

impl Hash for Vec3i {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl PartialEq for Vec3i {
    fn eq(&self, other: &Vec3i) -> bool { self.x == other.x && self.y == other.y && self.z == other.z }
}


impl Add for Vec3i { type Output = Self; fn add(self, o: Self) -> Self { Vec3i::new(self.x + o.x, self.y + o.y, self.z + o.z) } }
impl Mul for Vec3i { type Output = Self; fn mul(self, o: Self) -> Self { Vec3i::new(self.x * o.x, self.y * o.y, self.z * o.z) } }
impl Div for Vec3i { type Output = Self; fn div(self, o: Self) -> Self { Vec3i::new(self.x / o.x, self.y / o.y, self.z / o.z) } }
impl Sub for Vec3i { type Output = Self; fn sub(self, o: Self) -> Self { Vec3i::new(self.x - o.x, self.y - o.y, self.z - o.z) } }
impl Add<i32> for Vec3i { type Output = Self; fn add(self, o: i32) -> Self { Vec3i::new(self.x + o, self.y + o, self.z + o) } }
impl Mul<i32> for Vec3i { type Output = Self; fn mul(self, o: i32) -> Self { Vec3i::new(self.x * o, self.y * o, self.z * o) } }
impl Div<i32> for Vec3i { type Output = Self; fn div(self, o: i32) -> Self { Vec3i::new(self.x / o, self.y / o, self.z / o) } }
impl Sub<i32> for Vec3i { type Output = Self; fn sub(self, o: i32) -> Self { Vec3i::new(self.x - o, self.y - o, self.z - o) } }

impl AddAssign for Vec3i { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; self.z += o.z; } }
impl MulAssign for Vec3i { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; self.z *= o.z; } }
impl DivAssign for Vec3i { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; self.z /= o.z; } }
impl SubAssign for Vec3i { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; self.z -= o.z; } }
impl AddAssign<i32> for Vec3i { fn add_assign(&mut self, o: i32) { self.x += o; self.y += o; self.z += o; } }
impl MulAssign<i32> for Vec3i { fn mul_assign(&mut self, o: i32) { self.x *= o; self.y *= o; self.z *= o; } }
impl DivAssign<i32> for Vec3i { fn div_assign(&mut self, o: i32) { self.x /= o; self.y /= o; self.z /= o; } }
impl SubAssign<i32> for Vec3i { fn sub_assign(&mut self, o: i32) { self.x -= o; self.y -= o; self.z -= o; } }
