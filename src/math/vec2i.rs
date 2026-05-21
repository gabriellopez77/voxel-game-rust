use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub const ZERO: Vec2i = Vec2i { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self { Self { x, y, } }
    pub fn from1f(value: i32) -> Self { Self { x: value, y: value } }

    pub fn as_ptr(&self) -> *const i32 { &self.x }
}

impl PartialEq for Vec2i {
    fn eq(&self, other: &Vec2i) -> bool { self.x == other.x && self.y == other.y }
}


impl Add for Vec2i { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y, } } }
impl Mul for Vec2i { type Output = Self; fn mul(self, o: Self) -> Self { Self { x: self.x * o.x, y: self.y * o.y, } } }
impl Div for Vec2i { type Output = Self; fn div(self, o: Self) -> Self { Self { x: self.x / o.x, y: self.y / o.y, } } }
impl Sub for Vec2i { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y, } } }
impl Add<i32> for Vec2i { type Output = Self; fn add(self, o: i32) -> Self { Self { x: self.x + o, y: self.y + o, } } }
impl Mul<i32> for Vec2i { type Output = Self; fn mul(self, o: i32) -> Self { Self { x: self.x * o, y: self.y * o, } } }
impl Div<i32> for Vec2i { type Output = Self; fn div(self, o: i32) -> Self { Self { x: self.x / o, y: self.y / o, } } }
impl Sub<i32> for Vec2i { type Output = Self; fn sub(self, o: i32) -> Self { Self { x: self.x - o, y: self.y - o, } } }

impl AddAssign for Vec2i { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; } }
impl MulAssign for Vec2i { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; } }
impl DivAssign for Vec2i { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; } }
impl SubAssign for Vec2i { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; } }
impl AddAssign<i32> for Vec2i { fn add_assign(&mut self, o: i32) { self.x += o; self.y += o; } }
impl MulAssign<i32> for Vec2i { fn mul_assign(&mut self, o: i32) { self.x *= o; self.y *= o; } }
impl DivAssign<i32> for Vec2i { fn div_assign(&mut self, o: i32) { self.x /= o; self.y /= o; } }
impl SubAssign<i32> for Vec2i { fn sub_assign(&mut self, o: i32) { self.x -= o; self.y -= o; } }