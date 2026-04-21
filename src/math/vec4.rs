use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::SubAssign;

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

    pub fn from1f(value: f32) -> Self {
        return Vec4 {
            x: value,
            y: value,
            z: value,
            w: value
        }
    }

    pub fn from4f(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Vec4 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }
    
    pub fn as_ptr(&self) -> *const f32 { &self.x }
}


impl Add for Vec4 {
    type Output = Self; // Defines the return type

    fn add(self, other: Self) -> Self {
        return Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Mul for Vec4 {
    type Output = Self; // Defines the return type

    fn mul(self, other: Self) -> Self {
        return Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Div for Vec4 {
    type Output = Self; // Defines the return type

    fn div(self, other: Self) -> Self {
        return Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self; // Defines the return type

    fn sub(self, other: Self) -> Self {
        return Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Add<f32> for Vec4 {
    type Output = Self; // Defines the return type

    fn add(self, other: f32) -> Self {
        return Vec4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self; // Defines the return type

    fn mul(self, other: f32) -> Self {
        return Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self; // Defines the return type

    fn div(self, other: f32) -> Self {
        return Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self; // Defines the return type

    fn sub(self, other: f32) -> Self {
        return Vec4 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}


impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl DivAssign for Vec4 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl AddAssign<f32> for Vec4 {
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
        self.z += other;
        self.w += other;
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl SubAssign<f32> for Vec4 {
    fn sub_assign(&mut self, other: f32) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
        self.w -= other;
    }
}