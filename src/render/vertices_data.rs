use crate::math::*;
use crate::resources::TexCoords;


pub const SPRITES_VERTICES: [f32; 16] = [
    1.0, 1.0,  1.0, 1.0, // top right
    1.0, 0.0,  1.0, 0.0, // bottom right
    0.0, 0.0,  0.0, 0.0, // bottom left
    0.0, 1.0,  0.0, 1.0, // top left
];

pub const SPRITES_INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpritesVertices {
    pub position: Vec2i16,
    pub size: Vec2i16,
    pub uv: TexCoords,
    pub color: Color4b,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TextVertices {
    pub position: Vec2i16,
    pub size: Vec2u8,
    pub uv: TexCoords,
    pub advance: Vec2i16,
    pub color: Color3b,
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChunkVertices {
    pub vertices: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub flags: u8,
}

#[derive(Clone, Copy)]
pub struct BlockModelMesh {
    pub vertices: Vec3,
    pub uv: Vec2,
    pub normal: Vec3,
    pub shade: bool,
}
