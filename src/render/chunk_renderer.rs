use std::{cell::RefCell, rc::Rc, mem::offset_of};
use crate::math::{Vec3, Vec3i};
use crate::render::{render_utils, ChunkVertices, Shader, Texture, Vao, vao::VaoBuffers};
use crate::world::Chunk;
use crate::world::chunk::ChunkMeshResult;


#[repr(i32)]
#[derive(Copy, Clone)]
pub enum RendererType {
    Opaque,
    Alpha
}

pub struct ChunkRenderer {
    pub position: Vec3,

    vao: Vao,
    pub shader: Rc<RefCell<Shader>>,
    pub texture: Rc<Texture>,
}

impl ChunkRenderer {
    pub fn new(position: Vec3i, shader: Rc<RefCell<Shader>>, texture: Rc<Texture>) -> Self {
        let pos = Vec3 {
            x: (position.x * Chunk::CHUNK_SIZE.x) as f32,
            y: (position.y * Chunk::CHUNK_SIZE.y) as f32,
            z: (position.z * Chunk::CHUNK_SIZE.z) as f32
        };

        Self {
            position: pos,
            vao: Vao::new(),
            shader,
            texture,
        }
    }

    pub fn recreate(&mut self, position: Vec3i, shader: Rc<RefCell<Shader>>, texture: Rc<Texture>) {
        let pos = Vec3 {
            x: (position.x * Chunk::CHUNK_SIZE.x) as f32,
            y: (position.y * Chunk::CHUNK_SIZE.y) as f32,
            z: (position.z * Chunk::CHUNK_SIZE.z) as f32
        };

        self.position = pos;

        self.vao = Vao::new();
        self.shader = shader;
        self.texture = texture;
    }

    pub fn erase(&mut self) {
        self.vao.delete();
    }

    pub fn draw(&self) {
        if self.vao.triangles_count == 0 { return }

        self.shader.borrow_mut().set_vec3("pos", self.position);

        render_utils::draw_indexed(
            gl::TRIANGLES,
            &self.shader,
            Some(self.texture.as_ref()),
            &self.vao,
        );
    }

    pub fn update_mesh(&mut self, mesh_result: &ChunkMeshResult, render_type: RendererType) {
        let vertices = &mesh_result.vertices[render_type as usize];
        let indices = &mesh_result.indices[render_type as usize];

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
