use std::{cell::RefCell, rc::Rc, mem::offset_of};
use crate::math::{Vec3, Vec3i};
use crate::render::{render_utils, ChunkVertices, Shader, Texture, Vao, vao::VaoBuffers};
use crate::world::Chunk;


pub struct ChunkRenderer {
    position: Vec3,

    vao: Vao,
    shader: Rc<RefCell<Shader>>,
}

impl ChunkRenderer {
    pub fn new(position: Vec3i, shader: Rc<RefCell<Shader>>, ) -> Self {
        let pos = Vec3 {
            x: (position.x * Chunk::CHUNK_SIZE.x) as f32,
            y: (position.y * Chunk::CHUNK_SIZE.y) as f32,
            z: (position.z * Chunk::CHUNK_SIZE.z) as f32
        };
        
        Self {
            position: pos,
            vao: Vao::new(),
            shader
        }
    }

    pub fn erase(&mut self) {
        self.vao.delete();
    }

    pub fn draw(&mut self) {
        if self.vao.triangles_count == 0 { return }
        
        self.shader.borrow_mut().set_vec3("pos", self.position);
        
        render_utils::draw_indexed(
            gl::TRIANGLES,
            &self.shader.borrow(),
            None,
            &self.vao,
        );
    }

    pub fn update_mesh(&mut self, vertices: &Vec<ChunkVertices>, indices: &Vec<u32>) {
        if vertices.is_empty() { return }

        if !self.vao.is_generated() {
            self.vao.gen_vao()
                .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo)
                .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo);

            self.vao.buffer_data_from_arr(VaoBuffers::Ebo, &indices, gl::STATIC_DRAW);

            self.vao.buffer_data(VaoBuffers::Vbo, size_of::<ChunkVertices>() * vertices.len(), Some(vertices.as_ptr() as *const ()), gl::STATIC_DRAW)
                .attrib_info(0, 3, gl::FLOAT, offset_of!(ChunkVertices, vertices), false)
                .attrib_info(1, 3, gl::FLOAT, offset_of!(ChunkVertices, normal), false)
                .attrib_info(2, 2, gl::FLOAT, offset_of!(ChunkVertices, uv), false)
                .set_stride(size_of::<ChunkVertices>());
        }
        else {
            self.vao.smart_reallocate_buffer(VaoBuffers::Vbo, &vertices);
            self.vao.smart_reallocate_buffer(VaoBuffers::Ebo, &indices);
        }
    }
}