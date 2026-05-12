use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};


#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec4i16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16
}

impl Vec4i16 {
    pub const ZERO: Vec4i16 = Vec4i16 { x: 0, y: 0, z: 0, w: 0 };

    pub fn new(x: i16, y: i16, z: i16, w: i16) -> Self { Self { x, y, z, w } }
    pub fn from1f(value: i16) -> Self { Self { x: value, y: value, z: value, w: value } }
    
    pub fn as_ptr(&self) -> *const i16 { &self.x }
}


impl PartialEq for Vec4i16 {
    fn eq(&self, other: &Vec4i16) -> bool { self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w }
}

impl Add for Vec4i16 { type Output = Self; fn add(self, o: Self) -> Self { Vec4i16::new(self.x + o.x, self.y + o.y, self.z + o.z, self.w + o.w) } }
impl Mul for Vec4i16 { type Output = Self; fn mul(self, o: Self) -> Self { Vec4i16::new(self.x * o.x, self.y * o.y, self.z * o.z, self.w * o.w) } }
impl Div for Vec4i16 { type Output = Self; fn div(self, o: Self) -> Self { Vec4i16::new(self.x / o.x, self.y / o.y, self.z / o.z, self.w / o.w) } }
impl Sub for Vec4i16 { type Output = Self; fn sub(self, o: Self) -> Self { Vec4i16::new(self.x - o.x, self.y - o.y, self.z - o.z, self.w - o.w) } }
impl Add<i16> for Vec4i16 { type Output = Self; fn add(self, o: i16) -> Self { Vec4i16::new(self.x + o, self.y + o, self.z + o, self.w + o) } }
impl Mul<i16> for Vec4i16 { type Output = Self; fn mul(self, o: i16) -> Self { Vec4i16::new(self.x * o, self.y * o, self.z * o, self.w * o) } }
impl Div<i16> for Vec4i16 { type Output = Self; fn div(self, o: i16) -> Self { Vec4i16::new(self.x / o, self.y / o, self.z / o, self.w / o) } }
impl Sub<i16> for Vec4i16 { type Output = Self; fn sub(self, o: i16) -> Self { Vec4i16::new(self.x - o, self.y - o, self.z - o, self.w - o) } }

impl AddAssign for Vec4i16 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; self.z += o.z; self.w += o.w; } }
impl MulAssign for Vec4i16 { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; self.z *= o.z; self.w *= o.w; } }
impl DivAssign for Vec4i16 { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; self.z /= o.z; self.w /= o.w; } }
impl SubAssign for Vec4i16 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; self.z -= o.z; self.w -= o.w; } }
impl AddAssign<i16> for Vec4i16 { fn add_assign(&mut self, o: i16) { self.x += o; self.y += o; self.z += o; self.w += o; } }
impl MulAssign<i16> for Vec4i16 { fn mul_assign(&mut self, o: i16) { self.x *= o; self.y *= o; self.z *= o; self.w *= o; } }
impl DivAssign<i16> for Vec4i16 { fn div_assign(&mut self, o: i16) { self.x /= o; self.y /= o; self.z /= o; self.w /= o; } }
impl SubAssign<i16> for Vec4i16 { fn sub_assign(&mut self, o: i16) { self.x -= o; self.y -= o; self.z -= o; self.w -= o; } }