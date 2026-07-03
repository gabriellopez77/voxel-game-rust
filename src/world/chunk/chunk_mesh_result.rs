use std::{array, cell::RefCell};

use crate::{math::Vec3i, render::{ChunkVertices, chunk_renderer::RendererType}, utils::ObjectPool, world::chunk::{ChunkData, NeighborChunksData}};


pub struct ChunkMeshResult {
    pub neighbors_data: NeighborChunksData,
    pub chunk_data: Box<RefCell<ChunkData>>,

    pub vertices: [Vec<ChunkVertices>; RendererType::RENDERS_COUNT],
    pub indices: [Vec<u32>; RendererType::RENDERS_COUNT],

    pub chunk_pos: Vec3i,
}

impl ChunkMeshResult {
    pub fn new(chunk_data: Box<RefCell<ChunkData>>, neighbors_data: NeighborChunksData, vertices_pool: &mut ObjectPool<Vec<ChunkVertices>>,
               indices_pool: &mut ObjectPool<Vec<u32>>, chunk_pos: Vec3i) -> Self {
        Self {
            neighbors_data: neighbors_data,
            chunk_data: chunk_data,

            vertices: array::from_fn(|_| vertices_pool.get_or(|| Vec::new())),
            indices: array::from_fn(|_| indices_pool.get_or(|| Vec::new())),

            chunk_pos: chunk_pos,
        }
    }

    pub fn get_vertices(&mut self, render_type: RendererType) -> &mut Vec<ChunkVertices> {
        &mut self.vertices[render_type as usize]
    }

    pub fn gen_indices(&mut self) {
        for i in 0..RendererType::RENDERS_COUNT {
            let indices = &mut self.indices[i];
            let vertices = &self.vertices[i];

            if vertices.is_empty() { continue }

            let indices_count = vertices.len() / 4;

            if indices.capacity() < indices_count * 6 {
                indices.reserve(indices_count * 6);
            }

            let mut current_index: u32 = 0;

            for _ in 0..indices_count {
                indices.push(current_index + 0);
                indices.push(current_index + 1);
                indices.push(current_index + 3);

                indices.push(current_index + 1);
                indices.push(current_index + 2);
                indices.push(current_index + 3);

                current_index += 4;
            }
        }
    }

    pub fn restore(self, vertices_pool: &mut ObjectPool<Vec<ChunkVertices>>, indices_pool: &mut ObjectPool<Vec<u32>>,
                   chunk_data_pool: &mut ObjectPool<Box<RefCell<ChunkData>>>) {
        for mut vertices in self.vertices {
            vertices.clear();
            vertices_pool.restore(vertices);
        }

        for mut indices in self.indices {
            indices.clear();
            indices_pool.restore(indices);
        }

        self.neighbors_data.restore(chunk_data_pool);
    }
}
