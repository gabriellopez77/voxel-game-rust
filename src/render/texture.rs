use std::collections::HashMap;
use crate::render;
use crate::math::Vec4;

pub struct Texture {
    id: u32,
    textures_coords: HashMap<String, Vec4>
}

impl Texture {
    pub fn new() -> Self { Self { id: 0, textures_coords: HashMap::new() } }

    pub fn create_from_file(path: &str, filter: gl::types::GLenum) -> Self {
        let img = image::open(path).expect(format!("Failed to open: {}", path).as_str());

        return Self::create_from_pixels(img.as_rgba8().unwrap().as_raw(), img.width() as i32, img.height() as i32, filter);
    }

    pub fn create_from_pixels(pixels: &[u8], width: i32, height: i32, filter: gl::types::GLenum) -> Self {
        unsafe {
            let mut id: u32 = 0;

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);

            gl::TextureParameterf(id, gl::TEXTURE_MAG_FILTER, filter as f32);
            gl::TextureParameterf(id, gl::TEXTURE_MIN_FILTER, filter as f32);

            gl::TextureStorage2D(id, 1, gl::RGBA8, width, height);
            gl::TextureSubImage2D(id, 0, 0, 0, width, height,
                                  gl::RGBA, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const std::ffi::c_void);

            return Self { id, textures_coords: HashMap::new() };
        }
    }

    pub fn create_from_atlas(atlas_info: (Vec<u8>, Vec<(String, Vec4)>), width: i32, height: i32, filter: gl::types::GLenum) -> Self {
        let mut tex = Self::create_from_pixels(&atlas_info.0, width, height, filter);

        tex.textures_coords.reserve(atlas_info.1.len());
        
        for coords in atlas_info.1 {
            tex.textures_coords.insert(coords.0, coords.1);
        }

        return tex;
    }

    pub fn destroy(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }

        self.id = 0;
    }

    pub fn bind(&self) {
        render::render_utils::bind_texture(self.id);
    }
}