use crate::{render::{ChunkVertices, chunk_renderer::RendererType}, utils::ObjectPool};


pub struct ChunkMeshResult {
    pub vertices: [Vec<ChunkVertices>; RendererType::RENDERS_COUNT],
    pub indices: [Vec<u32>; RendererType::RENDERS_COUNT],
}

impl ChunkMeshResult {
    pub fn new(vertices_pool: &mut ObjectPool<Vec<ChunkVertices>>, indices_pool: &mut ObjectPool<Vec<u32>>) -> Self {
        fn get_vertices(pool: &mut ObjectPool<Vec<ChunkVertices>>) -> Vec<ChunkVertices> {
            match pool.get() {
                Some(x) => x,
                None => Vec::new()
            }
        }

        fn get_indices(pool: &mut ObjectPool<Vec<u32>>) -> Vec<u32> {
            match pool.get() {
                Some(x) => x,
                None => Vec::new()
            }
        }

        Self {
            vertices: [
                get_vertices(vertices_pool),
                get_vertices(vertices_pool),
            ],

            indices: [
                get_indices(indices_pool),
                get_indices(indices_pool),
            ],
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

    pub fn restore(self, vertices_pool: &mut ObjectPool<Vec<ChunkVertices>>, indices_pool: &mut ObjectPool<Vec<u32>>) {
        for mut vertices in self.vertices {
            vertices.clear();
            vertices_pool.insert(vertices);
        }

        for mut indices in self.indices {
            indices.clear();
            indices_pool.insert(indices);
        }
    }
}
