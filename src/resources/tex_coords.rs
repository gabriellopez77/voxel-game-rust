use crate::math::Vec2;


#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TexCoords {
    pub minx: f32,
    pub miny: f32,
    pub maxx: f32,
    pub maxy: f32,
}

impl TexCoords {
    pub const DEFAULT: Self = Self{ minx: 0.0, miny: 0.0, maxx: 1.0, maxy: 1.0 };
    pub const ZERO: Self = Self{ minx: 0.0, miny: 0.0, maxx: 0.0, maxy: 0.0 };

    pub const fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Self { Self { minx, miny, maxx, maxy } }

    pub fn newi(minx: i32, miny: i32, maxx: i32, maxy: i32) -> Self {
        TexCoords::new(
            minx as f32,
            miny as f32,
            maxx as f32,
            maxy as f32
        )
    }

    pub fn normalized(&self, atlas_size: Vec2) -> Self {
        TexCoords::new(
            self.minx / atlas_size.x,
            self.miny / atlas_size.y,
            self.maxx / atlas_size.x,
            self.maxy / atlas_size.y
        )
    }

    pub fn denormalized(&self, atlas_size: Vec2) -> Self {
        TexCoords::new(
            self.minx * atlas_size.x,
            self.miny * atlas_size.y,
            self.maxx * atlas_size.x,
            self.maxy * atlas_size.y
        )
    }

    /// no normalize!
    pub fn get_sub_tex(&self, x: f32, y: f32, width: f32, height: f32) -> Self {
        TexCoords::new(
            self.minx + x,
            self.miny + y,
            self.minx + (x + width),
            self.miny + (y + height)
        )
    }

    /// no normalize!
    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.maxx - self.minx, self.maxy - self.miny)
    }
}