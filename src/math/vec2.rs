use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::SubAssign;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn from1f(value: f32) -> Self {
        return Vec2 {
            x: value,
            y: value,
        }
    }

    pub fn as_ptr(&self) -> *const f32 { &self.x }
}


impl Add for Vec2 {
    type Output = Self; // Defines the return type

    fn add(self, other: Self) -> Self {
       return Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self; // Defines the return type

    fn mul(self, other: Self) -> Self {
       return Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self; // Defines the return type

    fn div(self, other: Self) -> Self {
       return Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self; // Defines the return type

    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Self; // Defines the return type

    fn add(self, other: f32) -> Self {
        return Vec2 {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self; // Defines the return type

    fn mul(self, other: f32) -> Self {
        return Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self; // Defines the return type

    fn div(self, other: f32) -> Self {
        return Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self; // Defines the return type

    fn sub(self, other: f32) -> Self {
        Vec2 {
            x: self.x - other,
            y: self.y - other,
        }
    }
}


impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, other: f32) {
        self.x += other;
        self.y += other;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, other: f32) {
        self.x -= other;
        self.y -= other;
    }
}