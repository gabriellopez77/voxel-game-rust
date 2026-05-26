use std::{cell::RefCell, rc::Rc};

use gl::types::GLenum;
use crate::render::{Shader, Texture, Vao};


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RenderCap {
    Blend,
    CullFace,
    DepthTest,
}

static mut CAPABILITIES: [u32; 3] = [0; 3];
const OPENGL_CAPABIILITIES: [u32; 3] = [gl::BLEND, gl::CULL_FACE, gl::DEPTH_TEST];

pub fn enable(cap: RenderCap) {
    let index = cap as usize;

    // SAFETY: call only in main thread
    unsafe {
        if CAPABILITIES[index] == 0 {
            CAPABILITIES[index] = OPENGL_CAPABIILITIES[index];
            gl::Enable(CAPABILITIES[index]);
        }
    }
}

pub fn disable(cap: RenderCap) {
    let index = cap as usize;

    // SAFETY: call only in main thread
    unsafe {
        if CAPABILITIES[index] != 0 {
            gl::Disable(CAPABILITIES[index]);
            CAPABILITIES[index] = 0;
        }
    }
}

pub fn draw_indexed(primitive: GLenum, shader: &Rc<RefCell<Shader>>, texture: Option<&Texture>, vao: &Vao) {
    unsafe {
        if let Some(texture) = texture { texture.bind() }
        shader.borrow().bind();
        vao.bind();

        gl::DrawElements(
            primitive,
            vao.triangles_count,
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }
}

pub fn draw_indexed_instanced(primitive: GLenum, shader: &Shader, texture: Option<&Texture>, vao: &Vao, instances_count: usize) {
    unsafe {
        if let Some(texture) = texture { texture.bind() }
        shader.bind();
        vao.bind();

        gl::DrawElementsInstanced(
            primitive,
            vao.triangles_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
            instances_count as i32
        );
    }
}

pub fn bind_buffer(buffer_type: GLenum, buffer_id: u32) {
    static mut CURRENT_BIND_BUFFER_ID: u32 = 0;
    static mut CURRENT_BIND_BUFFER_TYPE: GLenum = 0;

    // SAFETY: call only in main thread
    unsafe {
        // if current buffer type and id is the same that we want bind, then return
        if buffer_type == CURRENT_BIND_BUFFER_TYPE && buffer_id == CURRENT_BIND_BUFFER_ID { return }

        CURRENT_BIND_BUFFER_TYPE = buffer_type;
        CURRENT_BIND_BUFFER_ID = buffer_id;

        gl::BindBuffer(buffer_type, buffer_id);
    }
}

pub fn bind_texture(texture_id: u32) {
    static mut CURRENT_BIND_TEXTURE: u32 = 0;

    // SAFETY: call only in main thread
    unsafe {
        if CURRENT_BIND_TEXTURE == texture_id { return }

        CURRENT_BIND_TEXTURE = texture_id;

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
    }
}

pub fn bind_vao(vao_id: u32) {
    static mut CURRENT_BIND_VAO_ID: u32 = 0;

    // SAFETY: call only in main thread
    unsafe {
        if CURRENT_BIND_VAO_ID == vao_id { return }

        CURRENT_BIND_VAO_ID = vao_id;
        gl::BindVertexArray(vao_id);
    }
}

pub fn bind_shader(shader_id: u32) {
    static mut CURRENT_BIND_SHADER: u32 = 0;

    // SAFETY: call only in main thread
    unsafe{
        if CURRENT_BIND_SHADER == shader_id { return }

        CURRENT_BIND_SHADER = shader_id;

        gl::UseProgram(shader_id);
    }
}
