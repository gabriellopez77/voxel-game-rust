use std::mem::offset_of;
use gl::types::GLsizei;
use crate::render;
use crate::render::{SpritesVertices, Shader, Texture, Vao, vao::VaoBuffers, SPRITES_VERTICES, SPRITES_INDICES};
use crate::render::vao;
use crate::resources::array_buffer::ArrayBuffer;


pub const MAX_SPRITES: usize = 500;

pub struct SpritesRenderer {
    pub buffer: ArrayBuffer<SpritesVertices, MAX_SPRITES>,

    pub shader: Shader,
    texture: Texture,
    vao: Vao
}

impl SpritesRenderer {
    pub fn new() -> Self {
        Self {
            buffer: ArrayBuffer::new(),
            shader: Shader::new(),
            texture: Texture::new(),
            vao: Vao::new()
        }
    }

    pub fn start(&mut self) {
        self.vao.gen_vao()
            .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, vao::VaoBuffers::Ebo)
            .gen_buffer(gl::ARRAY_BUFFER, vao::VaoBuffers::Vbo)
            .gen_buffer(gl::ARRAY_BUFFER, vao::VaoBuffers::Instance);
        
        self.vao.buffer_data_from_arr(vao::VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        self.vao.buffer_data_from_arr(vao::VaoBuffers::Vbo, &SPRITES_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 4, gl::FLOAT, 0, false)
            .set_stride(4 * size_of::<f32>());

        self.vao.buffer_data(vao::VaoBuffers::Instance, size_of::<SpritesVertices>() * MAX_SPRITES, None, gl::DYNAMIC_DRAW)
            .attrib_info(1, 2, gl::SHORT, offset_of!(SpritesVertices, position), true)
            .attrib_info(2, 2, gl::SHORT, offset_of!(SpritesVertices, size), true)
            .attrib_info(3, 4, gl::FLOAT, offset_of!(SpritesVertices, uv), true)
            .attrib_info(4, 4, gl::UNSIGNED_BYTE, offset_of!(SpritesVertices, color), true)
            .set_stride(size_of::<SpritesVertices>());

        match self.shader.compile_from_disk(r"\vert.glsl", r"\frag.glsl") {
            Some(x) => panic!("{}", x),
            None => ()
        };

        self.texture.create("grass_block_side.png", gl::NEAREST);
    }

    pub fn draw(&mut self) {
        let buffer_len = self.buffer.len();

        if buffer_len == 0 { return }
        
        self.vao.update_buffer(vao::VaoBuffers::Instance, &self.buffer);
        render::draw_indexed_instanced(gl::TRIANGLES, &self.shader, Some(&self.texture), &self.vao, self.buffer.len() as GLsizei);
        
        self.buffer.clear()
    }
}