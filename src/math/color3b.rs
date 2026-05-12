#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color3b {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3b {
    pub const ZERO: Color3b = Color3b { r: 0, g: 0, b: 0 };

    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn from1(value: u8) -> Self { Self { r: value, g: value, b: value } }
    
    pub fn from_hex(hex: u32) -> Self {
        let r = (hex >> 16) as u8;
        let g = (hex >> 8) as u8;
        let b = (hex >> 0) as u8;

        return Self { r, g, b };
    }
}