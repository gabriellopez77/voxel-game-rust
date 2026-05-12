use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2u8 {
    pub x: u8,
    pub y: u8,
}

impl Vec2u8 {
    pub const ZERO: Vec2u8 = Vec2u8 { x: 0, y: 0 };

    pub fn new(x: u8, y: u8) -> Self { Self { x, y, } }
    pub fn from1f(value: u8) -> Self { Self { x: value, y: value } }

    pub fn as_ptr(&self) -> *const u8 { &self.x }
}

impl PartialEq for Vec2u8 {
    fn eq(&self, other: &Vec2u8) -> bool { self.x == other.x && self.y == other.y }
}


impl Add for Vec2u8 { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y, } } }
impl Mul for Vec2u8 { type Output = Self; fn mul(self, o: Self) -> Self { Self { x: self.x * o.x, y: self.y * o.y, } } }
impl Div for Vec2u8 { type Output = Self; fn div(self, o: Self) -> Self { Self { x: self.x / o.x, y: self.y / o.y, } } }
impl Sub for Vec2u8 { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y, } } }
impl Add<u8> for Vec2u8 { type Output = Self; fn add(self, o: u8) -> Self { Self { x: self.x + o, y: self.y + o, } } }
impl Mul<u8> for Vec2u8 { type Output = Self; fn mul(self, o: u8) -> Self { Self { x: self.x * o, y: self.y * o, } } }
impl Div<u8> for Vec2u8 { type Output = Self; fn div(self, o: u8) -> Self { Self { x: self.x / o, y: self.y / o, } } }
impl Sub<u8> for Vec2u8 { type Output = Self; fn sub(self, o: u8) -> Self { Self { x: self.x - o, y: self.y - o, } } }

impl AddAssign for Vec2u8 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; } }
impl MulAssign for Vec2u8 { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; } }
impl DivAssign for Vec2u8 { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; } }
impl SubAssign for Vec2u8 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; } }
impl AddAssign<u8> for Vec2u8 { fn add_assign(&mut self, o: u8) { self.x += o; self.y += o; } }
impl MulAssign<u8> for Vec2u8 { fn mul_assign(&mut self, o: u8) { self.x *= o; self.y *= o; } }
impl DivAssign<u8> for Vec2u8 { fn div_assign(&mut self, o: u8) { self.x /= o; self.y /= o; } }
impl SubAssign<u8> for Vec2u8 { fn sub_assign(&mut self, o: u8) { self.x -= o; self.y -= o; } }