use std::cell::RefCell;
use std::rc::Rc;
use crate::math::Vec3;
use crate::render::{render_utils, Shader, Vao, CUBE_INDICES, CUBE_VERTICES};
use crate::render::render_utils::RenderCap;
use crate::render::vao::VaoBuffers;
use crate::resources::ResourceManager;


pub struct SelectionBox {
    shader: Option<Rc<RefCell<Shader>>>,
    vao: Vao,
    
    visible: bool,
}

impl SelectionBox {
    pub fn new() -> Self {
        Self {
            shader: None,
            vao: Vao::new(),
            
            visible: false,
        }
    }

    pub fn start(&mut self, resources: &ResourceManager) {
        self.vao.gen_vao()
            .gen_buffer(VaoBuffers::Ebo)
            .gen_buffer(VaoBuffers::Vbo);
        
        self.vao.buffer_data_from_arr(VaoBuffers::Ebo, &CUBE_INDICES, gl::STATIC_DRAW);
        
        self.vao.buffer_data_from_arr(VaoBuffers::Vbo, &CUBE_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 3, gl::BYTE, 0, false)
            .set_stride(6);

        self.shader = resources.get_shader("selection_box");
    }
    
    pub fn update(&mut self, dt: f32, position: Option<Vec3>) {
        if let Some(position) = position {
            self.shader.as_ref().unwrap().borrow_mut().set_vec3("pos", position);
            self.visible = true;
        }
        else {
            self.visible = false;
        }
    }
    
    pub fn draw(&mut self) {
        if !self.visible { return }
        
        render_utils::enable(RenderCap::Blend);
        
        render_utils::draw_indexed(&self.shader.as_ref().unwrap(), None, &self.vao);
        
        render_utils::disable(RenderCap::Blend);
    }
}