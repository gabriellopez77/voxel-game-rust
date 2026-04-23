use crate::resources;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new() -> Self { Self {id: 0 } }

    pub fn gen_texture(&mut self, path: &str, filter: gl::types::GLenum) {
        let full_path = format!("{}\\{}", resources::path::TEXTURES_PATH.get().unwrap(), path);

        let img = image::open(full_path).expect(format!("Failed to open: {}", path).as_str());

        unsafe {
            let mut id: u32 = 0;

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);

            gl::TextureParameterf(id, gl::TEXTURE_MAG_FILTER, filter as f32);
            gl::TextureParameterf(id, gl::TEXTURE_MIN_FILTER, filter as f32);

            gl::TextureStorage2D(id, 1, gl::RGBA8, img.width() as i32,  img.height() as i32);
            gl::TextureSubImage2D(id,0, 0, 0,
                img.width() as i32,  img.height() as i32, gl::RGBA, gl::UNSIGNED_BYTE,
                img.as_rgba8().unwrap().as_ptr() as *const std::ffi::c_void);
            
            self.id = id;
        }
    }

    pub fn bind(&self) {
        static mut CURRENT_BIND_TEXTURE:u32 = 0;

        unsafe {
            if CURRENT_BIND_TEXTURE == self.id { return }

            CURRENT_BIND_TEXTURE = self.id;

            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}