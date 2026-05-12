use std::collections::HashMap;
use std::path::PathBuf;
use crate::render::render_utils;
use crate::math::{Vec2};
use crate::resources::{TextureCoords, texture_atlas};

pub struct Texture {
    id: u32,

    size: Vec2,

    textures_coords: HashMap<String, TextureCoords>
}

impl Texture {
    pub fn new() -> Self { 
        Self { 
            id: 0,
            size: Vec2::ZERO,
            textures_coords: HashMap::new() 
        } 
    }

    pub fn create_from_file(path: &str, filter: gl::types::GLenum) -> Self {
        let img = image::open(path).expect(&format!("Failed to open: {path}"));
        let pixels =img.as_rgba8().unwrap().as_raw();

        return Self::create_from_pixels(pixels, img.width() as i32, img.height() as i32, filter);
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

            return Self { 
                id,
                size: Vec2::new(width as f32, height as f32),
                textures_coords: HashMap::new() 
            }
        }
    }

    pub fn create_from_atlas(images: &Vec<PathBuf>, width: i32, height: i32, filter: gl::types::GLenum) -> Self {
        let now = std::time::Instant::now();
        let (pixels, coords) = texture_atlas::create(images, width, height);
        println!("{}", now.elapsed().as_micros());

        let mut tex = Self::create_from_pixels(&pixels, width, height, filter);

        tex.textures_coords.reserve(coords.len());

        // insert texCoords
        for (name, coords) in coords {
            tex.textures_coords.insert(name, coords);
        }

        // valid atlas
        assert!(tex.textures_coords.contains_key("error_404"), "atlas does not have error texture!");

        return tex;
    }

    pub fn destroy(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }

        self.id = 0;
    }

    pub fn get_coords(&self, name: &str) -> TextureCoords {
        if let Some(tex) = self.textures_coords.get(name) { return *tex; }

        // is guaranteed that atlas contains the 'error_404' texture coords
        return self.textures_coords["error_404"];
    }

    pub fn bind(&self) {
        render_utils::bind_texture(self.id);
    }

    pub fn get_size(&self) -> Vec2 { self.size }
}