#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TextureCoords {
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
}

impl TextureCoords {
    pub const DEFAULT: TextureCoords = TextureCoords{ minx: 0.0, miny: 0.0, maxx: 1.0, maxy: 1.0 };
    pub const ZERO: TextureCoords = TextureCoords{ minx: 0.0, miny: 0.0, maxx: 0.0, maxy: 0.0 };

    pub fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Self { Self { minx, miny, maxx, maxy } }

    pub fn newi(minx: i32, miny: i32, maxx: i32, maxy: i32) -> Self {
        Self {
            minx: minx as f32,
            miny: miny as f32,
            maxx: maxx as f32,
            maxy: maxy as f32 
        }
    }

    pub fn normalized(&self, atlas_width: f32, atlas_height: f32) -> Self {
        Self {
            minx: self.minx / atlas_width,
            miny: self.miny / atlas_height,
            maxx: self.maxx / atlas_width,
            maxy: self.maxy / atlas_height,
        }
    }
}