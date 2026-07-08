use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self { Self { x, y, } }
    pub fn from1(value: f32) -> Self { Self { x: value, y: value } }

    pub fn as_ptr(&self) -> *const f32 { &self.x }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool { self.x == other.x && self.y == other.y }
}


impl Add for Vec2 { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y, } } }
impl Mul for Vec2 { type Output = Self; fn mul(self, o: Self) -> Self { Self { x: self.x * o.x, y: self.y * o.y, } } }
impl Div for Vec2 { type Output = Self; fn div(self, o: Self) -> Self { Self { x: self.x / o.x, y: self.y / o.y, } } }
impl Sub for Vec2 { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y, } } }
impl Add<f32> for Vec2 { type Output = Self; fn add(self, o: f32) -> Self { Self { x: self.x + o, y: self.y + o, } } }
impl Mul<f32> for Vec2 { type Output = Self; fn mul(self, o: f32) -> Self { Self { x: self.x * o, y: self.y * o, } } }
impl Div<f32> for Vec2 { type Output = Self; fn div(self, o: f32) -> Self { Self { x: self.x / o, y: self.y / o, } } }
impl Sub<f32> for Vec2 { type Output = Self; fn sub(self, o: f32) -> Self { Self { x: self.x - o, y: self.y - o, } } }

impl AddAssign for Vec2 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; } }
impl MulAssign for Vec2 { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; } }
impl DivAssign for Vec2 { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; } }
impl SubAssign for Vec2 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; } }
impl AddAssign<f32> for Vec2 { fn add_assign(&mut self, o: f32) { self.x += o; self.y += o; } }
impl MulAssign<f32> for Vec2 { fn mul_assign(&mut self, o: f32) { self.x *= o; self.y *= o; } }
impl DivAssign<f32> for Vec2 { fn div_assign(&mut self, o: f32) { self.x /= o; self.y /= o; } }
impl SubAssign<f32> for Vec2 { fn sub_assign(&mut self, o: f32) { self.x -= o; self.y -= o; } }
