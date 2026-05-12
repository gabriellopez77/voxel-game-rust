use crate::math::*;
use crate::resources::TextureCoords;

pub const SPRITES_VERTICES: [f32; 16] = [
    1.0, 1.0,  1.0, 0.0, // top right
    1.0, 0.0,  1.0, 1.0, // bottom right
    0.0, 0.0,  0.0, 1.0, // bottom left
    0.0, 1.0,  0.0, 0.0, // top left
];

pub const SPRITES_INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpritesVertices {
    pub position: Vec2i16,
    pub size: Vec2i16,
    pub uv: TextureCoords,
    pub color: Color4b,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TextVertices {
    pub position: Vec2i16,
    pub size: Vec2u8,
    pub uv: Vec4i16,
    pub advance: Vec2i16,
    pub color: Color3b,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChunkVertices {
    pub vertices: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}


impl Default for SpritesVertices {
    fn default() -> Self {
        Self {
            position: Vec2i16::ZERO,
            size: Vec2i16::ZERO,
            uv: TextureCoords::ZERO,
            color: Color4b::ZERO,
        }
    }
}

impl Default for TextVertices {
    fn default() -> Self {
        Self {
            position: Vec2i16::ZERO,
            size: Vec2u8::ZERO,
            uv: Vec4i16::ZERO,
            advance: Vec2i16::ZERO,
            color: Color3b::ZERO,
        }
    }
}

impl Default for ChunkVertices {
    fn default() -> Self {
        Self {
            vertices: Vec3::ZERO,
            normal: Vec3::ZERO,
            uv: Vec2::ZERO,
        }
    }
}