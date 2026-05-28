use std::{cell::RefCell, rc::Rc};

use crate::render::render_utils;
use crate::render::{Shader, Texture, Vao, vao::VaoBuffers};


pub const MAX_SPRITES: usize = 500;

pub struct SpritesRenderer<T: Copy> {
    buffer: Vec<T>,

    shader: Option<Rc<RefCell<Shader>>>,
    texture: Option<Rc<Texture>>,
    vao: Vao
}

impl<T: Copy> SpritesRenderer<T> {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),

            shader: None,
            texture: None,
            vao: Vao::new()
        }
    }

    pub fn start(&mut self, vao: Vao, shader: Option<Rc<RefCell<Shader>>>, texture: Option<Rc<Texture>>) {
        self.vao = vao;
        self.texture = texture;
        self.shader = shader;
    }

    pub fn draw(&mut self) {
        let buffer_len = self.buffer.len();

        if buffer_len == 0 { return }

        self.vao.update_buffer(VaoBuffers::Instance, &self.buffer);
        render_utils::draw_indexed_instanced(
            gl::TRIANGLES,
            &self.shader.as_ref().unwrap().borrow(),
            Some(&self.texture.as_ref().unwrap()),
            &self.vao,
            buffer_len
        );

        self.buffer.clear()
    }

    pub fn buffer_len(&self) -> usize { self.buffer.len() }

    pub fn add_element(&mut self, element: T) { self.buffer.push(element) }
}
