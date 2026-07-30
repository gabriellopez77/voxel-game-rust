use std::{array, sync::{Arc, RwLock}};

use crate::{math::Vec3i, render::{ChunkVertices, chunk_renderer::RendererType}, world::{Chunk, Planet, chunk::{ChunkData, neighbors_data::NeighborsData}}};


pub struct ChunkMeshResult {
    pub neighbors_data: NeighborsData,
    pub chunk_data: Arc<RwLock<ChunkData>>,

    pub vertices: [Vec<ChunkVertices>; RendererType::RENDERS_COUNT],
    pub indices: [Vec<u32>; RendererType::RENDERS_COUNT],

    pub chunk_pos: Vec3i,
}

impl ChunkMeshResult {
    pub fn new(planet: &mut Planet, chunk: &Chunk) -> Self {
        Self {
            neighbors_data: NeighborsData::new(planet, chunk.position),
            chunk_data: chunk.chunk_data.clone(),

            vertices: array::from_fn(|_| planet.chunk_mesh_vertices_pool.get_or(|| Vec::new())),
            indices: array::from_fn(|_| planet.chunk_mesh_indices_pool.get_or(|| Vec::new())),

            chunk_pos: chunk.position,
        }
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

    pub fn restore(self, planet: &mut Planet) {
        for mut vertices in self.vertices {
            vertices.clear();
            planet.chunk_mesh_vertices_pool.restore(vertices);
        }

        for mut indices in self.indices {
            indices.clear();
            planet.chunk_mesh_indices_pool.restore(indices);
        }
    }
}
