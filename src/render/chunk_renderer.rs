use std::{cell::RefCell, rc::Rc, mem::offset_of};
use crate::math::{Vec3, Vec3i};
use crate::render::{render_utils, ChunkVertices, Shader, Texture, Vao, vao::VaoBuffers};
use crate::world::{Chunk, chunk::ChunkMeshResult};


#[derive(Copy, Clone)]
pub enum RendererType {
    Opaque,
    Alpha,
}

impl RendererType {
    pub const RENDERS_COUNT: usize = 2;
}

pub struct ChunkRenderer {
    pub position: Vec3,

    vaos: [Vao; RendererType::RENDERS_COUNT],
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
            vaos: [
                Vao::new(),
                Vao::new()
            ],
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

        self.vaos = [
            Vao::new(),
            Vao::new()
        ];
        self.shader = shader;
        self.texture = texture;
    }

    pub fn erase(&mut self) {
        for vao in &mut self.vaos {
            vao.delete();
        }
    }

    pub fn draw(&self, render_type: RendererType) {
        let vao = &self.vaos[render_type as usize];

        if vao.triangles_count == 0 { return }

        self.shader.borrow_mut().set_vec3("pos", self.position);

        render_utils::draw_indexed(
            gl::TRIANGLES,
            &self.shader,
            Some(self.texture.as_ref()),
            vao,
        );
    }

    pub fn update_mesh(&mut self, mesh_result: &ChunkMeshResult) {
        for i in 0..RendererType::RENDERS_COUNT {
            let vertices = &mesh_result.vertices[i];

            if vertices.is_empty() { return }

            let indices = &mesh_result.indices[i];
            let vao = &mut self.vaos[i];

            if !vao.is_generated() {
                vao.gen_vao()
                    .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo)
                    .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo);

                vao.buffer_data_from_arr(VaoBuffers::Ebo, &indices, gl::STATIC_DRAW);

                vao.buffer_data_from_arr(VaoBuffers::Vbo, &vertices, gl::STATIC_DRAW)
                    .attrib_info(0, 3, gl::FLOAT, offset_of!(ChunkVertices, vertices), false)
                    .attrib_info(1, 3, gl::FLOAT, offset_of!(ChunkVertices, normal), false)
                    .attrib_info(2, 2, gl::FLOAT, offset_of!(ChunkVertices, uv), false)
                    .attrib_info(3, 1, gl::UNSIGNED_BYTE, offset_of!(ChunkVertices, flags), false)
                    .set_stride(size_of::<ChunkVertices>());
            }
            else {
                vao.smart_reallocate_buffer(VaoBuffers::Ebo, &indices);
                vao.smart_reallocate_buffer(VaoBuffers::Vbo, &vertices);
            }
        }
    }
}
