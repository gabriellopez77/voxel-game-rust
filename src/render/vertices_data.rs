use crate::math::*;
use crate::resources::TexCoords;


pub const SPRITES_INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];

pub const SPRITES_VERTICES: [f32; 16] = [
    1.0, 1.0,  1.0, 1.0, // top right
    1.0, 0.0,  1.0, 0.0, // bottom right
    0.0, 0.0,  0.0, 0.0, // bottom left
    0.0, 1.0,  0.0, 1.0, // top left
];

pub const CUBE_INDICES: [u32; 36] = [
    0,  1,  3,  1,  2,  3, // up
    4,  5,  7,  5,  6,  7, // down
    8,  9,  11, 9,  10, 11, // south
    12, 13, 15, 13, 14, 15, // north
    16, 17, 19, 17, 18, 19, // west
    20, 21, 23, 21, 22, 23, // east
];

pub const CENTER_SPRITES_VERTICES: [f32; 20] = [
     0.5,  0.5,  0.0,   1.0, 1.0, // bottom right
     0.5, -0.5,  0.0,   1.0, 0.0, // top right
    -0.5, -0.5,  0.0,   0.0, 0.0, // top left
    -0.5,  0.5,  0.0,   0.0, 1.0, // bottom left
];

pub const PARTICLES_VERTICES: [f32; 20] = [
    0.0,  0.5,  0.5,   1.0, 0.0, // bottom right
    0.0,  0.5, -0.5,   0.0, 0.0, // top right
    0.0, -0.5, -0.5,   0.0, 1.0, // top left
    0.0, -0.5,  0.5,   1.0, 1.0, // bottom left
];

// vertices, normal
pub const CUBE_VERTICES: [i8; 144] = [
    // up
    1, 1, 0,   0, 1, 0,
    0, 1, 0,   0, 1, 0,
    0, 1, 1,   0, 1, 0,
    1, 1, 1,   0, 1, 0,

    // down
    1, 0, 1,   0, -1, 0,
    0, 0, 1,   0, -1, 0,
    0, 0, 0,   0, -1, 0,
    1, 0, 0,   0, -1, 0,

    // south
    0, 1, 1,   0, 0, 1,
    0, 0, 1,   0, 0, 1,
    1, 0, 1,   0, 0, 1,
    1, 1, 1,   0, 0, 1,

    // north
    1, 1, 0,   0, 0, -1,
    1, 0, 0,   0, 0, -1,
    0, 0, 0,   0, 0, -1,
    0, 1, 0,   0, 0, -1,

    // west
    0, 1, 0,  -1, 0, 0,
    0, 0, 0,  -1, 0, 0,
    0, 0, 1,  -1, 0, 0,
    0, 1, 1,  -1, 0, 0,

    // east
    1, 1, 1,   1, 0, 0,
    1, 0, 1,   1, 0, 0,
    1, 0, 0,   1, 0, 0,
    1, 1, 0,   1, 0, 0,
];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpritesVertices {
    pub position: Vec2i16,
    pub size: Vec2i16,
    pub uv: TexCoords,
    pub color: Color4b,
    pub texture_idx: u8,
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

    /// 00000011 = ao level
    /// 00000100 = shade flag
    pub flags: u8,
}

#[derive(Clone, Copy)]
pub struct BlockModelMesh {
    pub vertices: Vec3,
    pub uv: Vec2,
    pub normal: Vec3,
    pub shade: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CloudsVertices {
    pub position: Vec2,
    pub cullface: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SkyBodiesVertices {
    pub matrix: Matrix4,
    pub uv: TexCoords,
    pub color: Vec4,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ParticlesVertices {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
    pub uv: TexCoords,
    pub texture_idx: u8,
}
