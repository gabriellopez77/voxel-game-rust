use std::cell::RefCell;
use std::mem::offset_of;
use std::rc::Rc;
use crate::math::Vec4;
use crate::render::{render_utils, Shader, SkyBodiesVertices, Texture, Vao, CENTER_SPRITES_VERTICES, SPRITES_INDICES};
use crate::render::vao::VaoBuffers;


pub struct SkyBodiesRenderer {
    vao: Vao,

    instances_count: usize,
}

impl SkyBodiesRenderer {
    pub fn new() -> Self {
        Self {
            vao: Vao::new(),

            instances_count: 0,
        }
    }

    pub fn start(&mut self, buffer: &[SkyBodiesVertices]) {
        self.instances_count = buffer.len();
        
        self.vao.gen_vao()
            .gen_buffer(VaoBuffers::Ebo)
            .gen_buffer(VaoBuffers::Vbo)
            .gen_buffer(VaoBuffers::Instance);

        self.vao.buffer_data_from_arr(VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        self.vao.buffer_data_from_arr(VaoBuffers::Vbo, &CENTER_SPRITES_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 3, gl::FLOAT, 0, false)
            .attrib_info(1, 2, gl::FLOAT, 3 * size_of::<f32>(), false)
            .set_stride(5 * size_of::<f32>());

        self.vao.buffer_data_from_arr(VaoBuffers::Instance, buffer, gl::STATIC_DRAW)
            .attrib_info(2, 4, gl::FLOAT, 0, true)
            .attrib_info(3, 4, gl::FLOAT, 1 * size_of::<Vec4>(), true)
            .attrib_info(4, 4, gl::FLOAT, 2 * size_of::<Vec4>(), true)
            .attrib_info(5, 4, gl::FLOAT, 3 * size_of::<Vec4>(), true)
            .attrib_info(6, 4, gl::FLOAT, offset_of!(SkyBodiesVertices, uv), true)
            .attrib_info(7, 4, gl::FLOAT, offset_of!(SkyBodiesVertices, color), true)
            .set_stride(size_of::<SkyBodiesVertices>());
    }

    pub fn draw(&self, shader: &Rc<RefCell<Shader>>, texture: &Rc<Texture>) {
        render_utils::draw_indexed_instanced(
            shader,
            Some(texture.as_ref()),
            &self.vao,
            self.instances_count
        )
    }
}