use std::{array, sync::{Arc, RwLock}};

use crate::{math::Vec3i, render::{ChunkVertices, chunks_renderer::ChunksRendererType}, resources::ResourceManager, world::{Chunk, Planet, chunk::{ChunkData, neighbors_data::NeighborsData}}};


pub struct ChunkMeshResult {
    pub neighbors_data: NeighborsData,
    pub chunk_data: Arc<RwLock<ChunkData>>,

    pub vertices: [Vec<ChunkVertices>; ChunksRendererType::RENDERS_COUNT],
    pub indices: [Vec<u32>; ChunksRendererType::RENDERS_COUNT],

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
        for i in 0..ChunksRendererType::RENDERS_COUNT {
            let vertices = &self.vertices[i];

            if vertices.is_empty() { continue }

            ResourceManager::gen_indices(vertices.len(), &mut self.indices[i]);
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
