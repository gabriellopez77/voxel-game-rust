use std::{collections::HashMap, path::PathBuf};
use ash::vk;

use crate::math::{Vec2};
use crate::render::{RawTexture, VulkanApp};
use crate::resources::{TexCoords, texture_atlas};


pub struct Texture {
    pub raw_texture: RawTexture,

    size: Vec2,

    textures_coords: HashMap<String, (Vec2, TexCoords)>
}

impl Texture {
    pub fn new() -> Self {
        Self {
            raw_texture: RawTexture::new(),

            size: Vec2::ZERO,

            textures_coords: HashMap::new(),
        }
    }

    pub fn create_from_file(app: &mut VulkanApp, path: &str) -> Self {
        let img = image::open(path).expect(&format!("Failed to open: {path}"));
        let pixels =img.as_rgba8().unwrap().as_raw();

        return Self::create_from_pixels(app, pixels, img.width() as i32, img.height() as i32);
    }

    pub fn create_from_pixels(app: &mut VulkanApp, pixels: &[u8], width: i32, height: i32) -> Self {
        let mut raw_texture = RawTexture::new();
        raw_texture.create(app, width as u32, height as u32, pixels, vk::Filter::NEAREST, vk::SamplerAddressMode::REPEAT);

        return Self {
            raw_texture,

            size: Vec2::new(width as f32, height as f32),
            textures_coords: HashMap::new()
        }
    }

    pub fn create_from_atlas(app: &mut VulkanApp, images: &[PathBuf], width: i32, height: i32) -> Self {
        let (pixels, coords) = texture_atlas::create(images, width, height);

        let mut tex = Self::create_from_pixels(app, &pixels, width, height);

        tex.textures_coords.reserve(coords.len());

        // insert texCoords
        for (name, coords) in coords {
            tex.textures_coords.insert(name, coords);
        }

        // valided atlas
        assert!(tex.textures_coords.contains_key("error_404"), "atlas does not have error texture!");

        return tex;
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        self.raw_texture.destroy(app);
    }

    /// return normalized tex coords
    pub fn get_coords(&self, name: &str) -> TexCoords {
        if let Some(tex) = self.textures_coords.get(name) { return tex.1; }

        // is guaranteed that atlas contains the 'error_404'
        return self.textures_coords["error_404"].1;
    }

    pub fn get_atlas_tex_size(&self, name: &str) -> Vec2 {
        if let Some(tex) = self.textures_coords.get(name) { return tex.0; }

        // is guaranteed that atlas contains the 'error_404
        return self.textures_coords["error_404"].0;
    }

    pub fn get_size(&self) -> Vec2 { self.size }
}
