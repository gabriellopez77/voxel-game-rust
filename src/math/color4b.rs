#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color4b {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color4b {
    pub const ZERO: Color4b = Color4b { r: 0, g: 0, b: 0, a: 0 };

    pub fn from1(value: u8) -> Self { Self { r: value, g: value, b: value, a: value } }
    
    pub fn from_hex(hex: u32) -> Self {
        let r = (hex >> 24) as u8;
        let g = (hex >> 16) as u8;
        let b = (hex >> 8) as u8;
        let a = (hex >> 0) as u8;

        return Self {r, g, b, a };
    }
}