use std::ops::{Add, Mul, Div, Sub, AddAssign, MulAssign, DivAssign, SubAssign};


#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub const ZERO: Vec4 = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
    pub fn from1f(value: f32) -> Self { Self { x: value, y: value, z: value, w: value } }
    
    pub fn as_ptr(&self) -> *const f32 { &self.x }
}


impl PartialEq for Vec4 {
    fn eq(&self, other: &Vec4) -> bool { self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w }
}

impl Add for Vec4 { type Output = Self; fn add(self, o: Self) -> Self { Vec4::new(self.x + o.x, self.y + o.y, self.z + o.z, self.w + o.w) } }
impl Mul for Vec4 { type Output = Self; fn mul(self, o: Self) -> Self { Vec4::new(self.x * o.x, self.y * o.y, self.z * o.z, self.w * o.w) } }
impl Div for Vec4 { type Output = Self; fn div(self, o: Self) -> Self { Vec4::new(self.x / o.x, self.y / o.y, self.z / o.z, self.w / o.w) } }
impl Sub for Vec4 { type Output = Self; fn sub(self, o: Self) -> Self { Vec4::new(self.x - o.x, self.y - o.y, self.z - o.z, self.w - o.w) } }
impl Add<f32> for Vec4 { type Output = Self; fn add(self, o: f32) -> Self { Vec4::new(self.x + o, self.y + o, self.z + o, self.w + o) } }
impl Mul<f32> for Vec4 { type Output = Self; fn mul(self, o: f32) -> Self { Vec4::new(self.x * o, self.y * o, self.z * o, self.w * o) } }
impl Div<f32> for Vec4 { type Output = Self; fn div(self, o: f32) -> Self { Vec4::new(self.x / o, self.y / o, self.z / o, self.w / o) } }
impl Sub<f32> for Vec4 { type Output = Self; fn sub(self, o: f32) -> Self { Vec4::new(self.x - o, self.y - o, self.z - o, self.w - o) } }

impl AddAssign for Vec4 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; self.z += o.z; self.w += o.w; } }
impl MulAssign for Vec4 { fn mul_assign(&mut self, o: Self) { self.x *= o.x; self.y *= o.y; self.z *= o.z; self.w *= o.w; } }
impl DivAssign for Vec4 { fn div_assign(&mut self, o: Self) { self.x /= o.x; self.y /= o.y; self.z /= o.z; self.w /= o.w; } }
impl SubAssign for Vec4 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; self.z -= o.z; self.w -= o.w; } }
impl AddAssign<f32> for Vec4 { fn add_assign(&mut self, o: f32) { self.x += o; self.y += o; self.z += o; self.w += o; } }
impl MulAssign<f32> for Vec4 { fn mul_assign(&mut self, o: f32) { self.x *= o; self.y *= o; self.z *= o; self.w *= o; } }
impl DivAssign<f32> for Vec4 { fn div_assign(&mut self, o: f32) { self.x /= o; self.y /= o; self.z /= o; self.w /= o; } }
impl SubAssign<f32> for Vec4 { fn sub_assign(&mut self, o: f32) { self.x -= o; self.y -= o; self.z -= o; self.w -= o; } }