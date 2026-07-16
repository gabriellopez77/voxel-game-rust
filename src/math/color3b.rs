use std::ops::Mul;

use crate::math::Vec3;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color3b {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3b {
    pub const ZERO: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn from1(value: u8) -> Self { Self { r: value, g: value, b: value } }

    pub fn from_hex(hex: u32) -> Self {
        let r = (hex >> 16) as u8;
        let g = (hex >> 8) as u8;
        let b = (hex >> 0) as u8;

        return Self { r, g, b };
    }

    pub fn normalized(&self) -> Vec3 {
        Vec3::new(self.r as f32, self.g as f32, self.b as f32) / 255.0
    }
}

impl PartialEq for Color3b {
    fn eq(&self, other: &Self) -> bool {
        other.r == self.r && other.g == self.g && other.b == self.b
    }
}

impl Mul for Color3b {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let result = self.normalized() * other.normalized();

        return Self::new(
            (result.x * 255.0) as u8,
            (result.y * 255.0) as u8,
            (result.z * 255.0) as u8,
        );
    }
}
