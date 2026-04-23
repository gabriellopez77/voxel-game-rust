use std::mem::offset_of;
use gl::types::GLsizei;
use crate::render;
use crate::render::{SpritesVertices, Shader, Texture, Vao, SPRITES_VERTICES, SPRITES_INDICES};
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
        self.vao.gen_vao();
        self.vao.gen_buffer(gl::ELEMENT_ARRAY_BUFFER, vao::VaoBuffers::Ebo);
        self.vao.gen_buffer(gl::ARRAY_BUFFER, vao::VaoBuffers::Vbo);
        self.vao.gen_buffer(gl::ARRAY_BUFFER, vao::VaoBuffers::Instance);

        self.vao.bind();

        self.vao.bind_buffer(vao::VaoBuffers::Ebo);
        self.vao.buffer_data_from_arr(vao::VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        self.vao.bind_buffer(vao::VaoBuffers::Vbo);
        self.vao.buffer_data_from_arr(vao::VaoBuffers::Vbo, &SPRITES_VERTICES, gl::STATIC_DRAW);

        self.vao.attrib_info(0, 4, gl::FLOAT, 4 * size_of::<f32>(), 0, false);

        self.vao.bind_buffer(vao::VaoBuffers::Instance);
        self.vao.buffer_data(vao::VaoBuffers::Instance, size_of::<SpritesVertices>() * MAX_SPRITES, None, gl::DYNAMIC_DRAW);

        self.vao.attrib_info(1, 2, gl::SHORT, size_of::<SpritesVertices>(), offset_of!(SpritesVertices, position), true);
        self.vao.attrib_info(2, 2, gl::SHORT, size_of::<SpritesVertices>(), offset_of!(SpritesVertices, size), true);
        self.vao.attrib_info(3, 4, gl::FLOAT, size_of::<SpritesVertices>(), offset_of!(SpritesVertices, uv), true);
        self.vao.attrib_info(4, 4, gl::UNSIGNED_BYTE, size_of::<SpritesVertices>(), offset_of!(SpritesVertices, color), true);


        match self.shader.compile_from_disk(r"\vert.glsl", r"\frag.glsl") {
            Some(x) => panic!("{}", x),
            None => ()
        };

        self.texture.gen_texture("grass_block_side.png", gl::NEAREST);
    }

    pub fn draw(&mut self) {
        let buffer_len = self.buffer.len();

        if buffer_len == 0 { return }
        
        self.vao.update_buffer(vao::VaoBuffers::Instance, &self.buffer);
        render::draw_indexed_instanced(gl::TRIANGLES, &self.shader, Some(&self.texture), &self.vao, self.buffer.len() as GLsizei);
        
        self.buffer.clear()
    }
}