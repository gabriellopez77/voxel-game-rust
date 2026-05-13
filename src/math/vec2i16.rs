use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2i16 {
    pub x: i16,
    pub y: i16,
}

impl Vec2i16 {
    pub const ZERO: Vec2i16 = Vec2i16 { x: 0, y: 0 };

    pub fn new(x: i16, y: i16) -> Self { Self { x, y } }
    pub fn from1(value: i16) -> Self { Self{ x: value, y: value } }

    pub fn as_ptr(&self) -> *const i16 { &self.x }
}

impl PartialEq for Vec2i16 {
    fn eq(&self, other: &Vec2i16) -> bool { self.x == other.x && self.y == other.y }
}

impl Add for Vec2i16 { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y, } } }
impl Mul for Vec2i16 { type Output = Self; fn mul(self, o: Self) -> Self { Self { x: self.x * o.x, y: self.y * o.y, } } }
impl Div for Vec2i16 { type Output = Self; fn div(self, o: Self) -> Self { Self { x: self.x / o.x, y: self.y / o.y, } } }
impl Sub for Vec2i16 { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y, } } }
impl Add<i16> for Vec2i16 { type Output = Self; fn add(self, o: i16) -> Self { Self { x: self.x + o, y: self.y + o, } } }
impl Mul<i16> for Vec2i16 { type Output = Self; fn mul(self, o: i16) -> Self { Self { x: self.x * o, y: self.y * o, } } }
impl Div<i16> for Vec2i16 { type Output = Self; fn div(self, o: i16) -> Self { Self { x: self.x / o, y: self.y / o, } } }
impl Sub<i16> for Vec2i16 { type Output = Self; fn sub(self, o: i16) -> Self { Self { x: self.x - o, y: self.y - o, } } }

impl AddAssign for Vec2i16 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; } }
impl MulAssign for Vec2i16 { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; } }
impl DivAssign for Vec2i16 { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; } }
impl SubAssign for Vec2i16 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; } }
impl AddAssign<i16> for Vec2i16 { fn add_assign(&mut self, o: i16) { self.x += o; self.y += o; } }
impl MulAssign<i16> for Vec2i16 { fn mul_assign(&mut self, o: i16) { self.x *= o; self.y *= o; } }
impl DivAssign<i16> for Vec2i16 { fn div_assign(&mut self, o: i16) { self.x /= o; self.y /= o; } }
impl SubAssign<i16> for Vec2i16 { fn sub_assign(&mut self, o: i16) { self.x -= o; self.y -= o; } }