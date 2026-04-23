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
pub struct Vec2i16 {
    pub x: i16,
    pub y: i16,
}

impl Vec2i16 {
    pub const ZERO: Vec2i16 = Vec2i16 { x: 0, y: 0 };

    pub fn from1f(value: i16) -> Self {
        return Vec2i16 {
            x: value,
            y: value,
        }
    }

    pub fn as_ptr(&self) -> *const i16 { &self.x }
}


impl Add for Vec2i16 {
    type Output = Self; // Defines the return type

    fn add(self, other: Self) -> Self {
        return Vec2i16 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vec2i16 {
    type Output = Self; // Defines the return type

    fn mul(self, other: Self) -> Self {
        return Vec2i16 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vec2i16 {
    type Output = Self; // Defines the return type

    fn div(self, other: Self) -> Self {
        return Vec2i16 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Sub for Vec2i16 {
    type Output = Self; // Defines the return type

    fn sub(self, other: Self) -> Self {
        Vec2i16 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<i16> for Vec2i16 {
    type Output = Self; // Defines the return type

    fn add(self, other: i16) -> Self {
        return Vec2i16 {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Mul<i16> for Vec2i16 {
    type Output = Self; // Defines the return type

    fn mul(self, other: i16) -> Self {
        return Vec2i16 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<i16> for Vec2i16 {
    type Output = Self; // Defines the return type

    fn div(self, other: i16) -> Self {
        return Vec2i16 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Sub<i16> for Vec2i16 {
    type Output = Self; // Defines the return type

    fn sub(self, other: i16) -> Self {
        Vec2i16 {
            x: self.x - other,
            y: self.y - other,
        }
    }
}


impl AddAssign for Vec2i16 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl MulAssign for Vec2i16 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl DivAssign for Vec2i16 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl SubAssign for Vec2i16 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl AddAssign<i16> for Vec2i16 {
    fn add_assign(&mut self, other: i16) {
        self.x += other;
        self.y += other;
    }
}

impl MulAssign<i16> for Vec2i16 {
    fn mul_assign(&mut self, other: i16) {
        self.x *= other;
        self.y *= other;
    }
}

impl DivAssign<i16> for Vec2i16 {
    fn div_assign(&mut self, other: i16) {
        self.x /= other;
        self.y /= other;
    }
}

impl SubAssign<i16> for Vec2i16 {
    fn sub_assign(&mut self, other: i16) {
        self.x -= other;
        self.y -= other;
    }
}