use std::{cell::RefCell, rc::Rc};

use crate::{math::Vec3, render::{Shader, Ubo, Vao, render_utils}, resources::{ResourceManager, resources_manager}};

pub struct Sky {
    shader: Option<Rc<RefCell<Shader>>>,
    vao: Vao,
    ubo: Option<Rc<Ubo>>,
}

impl Sky {
    pub fn new() -> Self {
        Self {
            shader: None,
            vao: Vao::new(),
            ubo: None,
        }
    }
    pub fn start(&mut self, resources_manager: Rc<RefCell<ResourceManager>>) {
        let (vertices, indices) = resources_manager::gen_sphere(16.0, 16.0);

        let mut vao = Vao::new();

        vao.gen_vao()
            .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, crate::render::vao::VaoBuffers::Ebo)
            .gen_buffer(gl::ARRAY_BUFFER, crate::render::vao::VaoBuffers::Vbo);

        vao.buffer_data_from_arr(crate::render::vao::VaoBuffers::Ebo, &indices, gl::STATIC_DRAW);

        vao.buffer_data_from_arr(crate::render::vao::VaoBuffers::Vbo, &vertices, gl::STATIC_DRAW)
            .attrib_info(0, 3, gl::FLOAT, 0, false)
            .set_stride(size_of::<Vec3>());

        self.vao = vao;
        self.shader = resources_manager.borrow().get_shader("skyDome");

        let sky_color = Vec3::new(5.0, 94.0, 255.0) / 255.0;
        let fog_color = Vec3::new(128.0, 204.0, 255.0) / 255.0;
        self.ubo = resources_manager.borrow().get_ubo("worldData");
        self.ubo.as_ref().unwrap().update("fogColor", &fog_color);
        self.ubo.as_ref().unwrap().update("skyColor", &sky_color);
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&mut self) {
        unsafe { gl::Disable(gl::DEPTH_TEST) }
        render_utils::draw_indexed(
            gl::TRIANGLES,
            &self.shader.as_ref().unwrap(),
            None,
            &self.vao
        );
        unsafe { gl::Enable(gl::DEPTH_TEST) }
    }
}
