use crate::math::{Color3b, Vec4};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color4b {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color4b {
    pub const ZERO: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    pub fn from1(value: u8) -> Self { Self { r: value, g: value, b: value, a: value } }
    pub fn from3(rgb: Color3b, a: u8) -> Self { Self { r: rgb.r, g: rgb.g, b: rgb.b, a, } }

    pub fn from_hex(hex: u32) -> Self {
        let r = (hex >> 24) as u8;
        let g = (hex >> 16) as u8;
        let b = (hex >> 8) as u8;
        let a = (hex >> 0) as u8;

        return Self {r, g, b, a };
    }

    pub fn normalized(&self) -> Vec4 {
        Vec4::new(self.r as f32, self.g as f32, self.b as f32, self.a as f32) / 255.0
    }
}

impl PartialEq for Color4b {
    fn eq(&self, other: &Self) -> bool {
        other.r == self.r && other.g == self.g && other.b == self.b && other.a == self.a
    }
}
