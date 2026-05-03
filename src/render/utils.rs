use gl::types::GLenum;
use crate::render::shader::Shader;
use crate::render::texture::Texture;
use crate::render::vao::Vao;


pub fn draw_indexed(primitive: GLenum, shader: &Shader, texture: Option<&Texture>, vao: &Vao) {
    unsafe {
        if let Some(texture) = texture { texture.bind() }
        shader.bind();
        vao.bind();
        
        gl::DrawElements(primitive, vao.triangles_count, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub fn draw_indexed_instanced(primitive: GLenum, shader: &Shader, texture: Option<&Texture>, vao: &Vao, instances_count: i32) {
    unsafe {
        if let Some(texture) = texture { texture.bind() }
        shader.bind();
        vao.bind();

        gl::DrawElementsInstanced(primitive, vao.triangles_count, gl::UNSIGNED_INT, std::ptr::null(), instances_count);
    }
}

pub fn bind_buffer(buffer_type: GLenum, buffer_id: u32) {
    static mut CURRENT_BIND_BUFFER_ID: u32 = 0;
    static mut CURRENT_BIND_BUFFER_TYPE: GLenum = 0;

    // unsafe block is safe because this func can just be call in main thread
    unsafe {
        // if current buffer type and id is the same that we want bind, then return
        if buffer_type == CURRENT_BIND_BUFFER_TYPE && buffer_id == CURRENT_BIND_BUFFER_ID { return }

        CURRENT_BIND_BUFFER_TYPE = buffer_type;
        CURRENT_BIND_BUFFER_ID = buffer_id;

        gl::BindBuffer(buffer_type, buffer_id);
    }
}

pub fn bind_texture(texture_id: u32) {
    static mut CURRENT_BIND_TEXTURE:u32 = 0;

    unsafe {
        if CURRENT_BIND_TEXTURE == texture_id { return }

        CURRENT_BIND_TEXTURE = texture_id;

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
    }
}