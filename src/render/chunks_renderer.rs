use std::{array, cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, RwLock}};

use crate::{math::Vec3i, render::{ChunkVertices, GlobalRenderer, Material, MultiMesh, core::raw_buffer::BufferFlags, multi_mesh::MultiMeshInfo}, resources::{ResourceManager, ThreadWorkerValue}, utils::NullSafePtr, world::{Chunk, blocks::BlocksManager, chunk::{ChunkData, NeighborsChunksData}}};
use crate::utils::ObjectPool;


pub struct ChunkMeshResult {
    pub neighbors_data: NeighborsChunksData,
    pub chunk_data: Arc<RwLock<ChunkData>>,

    pub vertices: [Vec<ChunkVertices>; ChunksRendererType::RENDERS_COUNT],
    pub indices: [Vec<u32>; ChunksRendererType::RENDERS_COUNT],

    pub chunk_pos: Vec3i,
}

impl ChunkMeshResult {
    pub fn gen_indices(&mut self) {
        for i in 0..ChunksRendererType::RENDERS_COUNT {
            let vertices = &self.vertices[i];

            if vertices.is_empty() { continue }

            ResourceManager::gen_indices(vertices.len(), &mut self.indices[i]);
        }
    }
}

#[derive(Copy, Clone)]
pub enum ChunksRendererType {
    Opaque,
    Alpha,
}

impl ChunksRendererType {
    pub const RENDERS_COUNT: usize = 2;
}

pub struct ChunksRenderer {
    multi_mesh: Option<MultiMesh>,

    materials: Option<[Rc<RefCell<Material>>; ChunksRendererType::RENDERS_COUNT]>,

    generated_mesh: HashMap<Vec3i, ChunkMeshResult>,

    mesh_gen_worker: ThreadWorkerValue<ChunkMeshResult, 1>,

    chunk_mesh_vertices_pool: ObjectPool<Vec<ChunkVertices>>,
    chunk_mesh_indices_pool: ObjectPool<Vec<u32>>,

    blocks_manager: NullSafePtr<BlocksManager>,
}

impl ChunksRenderer {
    pub fn new() -> Self {
        Self {
            multi_mesh: None,

            materials: None,

            generated_mesh: HashMap::new(),

            mesh_gen_worker: ThreadWorkerValue::new(),

            chunk_mesh_vertices_pool: ObjectPool::new(),
            chunk_mesh_indices_pool: ObjectPool::new(),

            blocks_manager: NullSafePtr::null(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager, global_renderer: &mut GlobalRenderer) {
        let mut multi_mesh =  global_renderer.create_multi_mesh(size_of::<ChunkVertices>());
        multi_mesh.start(BufferFlags::VRAM | BufferFlags::RARE_UPDATE);
        multi_mesh.create_profile(BufferFlags::RAM);

        self.multi_mesh = Some(multi_mesh);

        self.materials = Some([
            global_renderer.get_material("chunks"),
            global_renderer.get_material("chunksAlpha"),
        ]);

        self.mesh_gen_worker.start();
        self.blocks_manager = NullSafePtr::new(blocks_manager);
    }

    pub fn cleanup(&mut self) {
        self.multi_mesh.as_mut().unwrap().destroy();

        self.chunk_mesh_indices_pool.clear();
        self.chunk_mesh_vertices_pool.clear();
    }

    pub fn update_mesh(&mut self, info: &mut MultiMeshInfo, mesh_result: &ChunkMeshResult, render_type: ChunksRendererType) {
        let multi_mesh = self.multi_mesh.as_mut().unwrap();

        multi_mesh.remove_mesh(info);

        *info = multi_mesh.add_mesh(
            &mesh_result.vertices[render_type as usize],
            &mesh_result.indices[render_type as usize],
        );
    }

    pub fn dispose_mesh(&mut self, info: &mut MultiMeshInfo) {
        self.multi_mesh.as_mut().unwrap().remove_mesh(info);
    }

    pub fn record_draw(&mut self, info: MultiMeshInfo, render_type: ChunksRendererType) {
        let multi_mesh = self.multi_mesh.as_mut().unwrap();

        multi_mesh.record_mesh_info(info, render_type as usize);
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        let multi_mesh = self.multi_mesh.as_mut().unwrap();

        for i in 0..ChunksRendererType::RENDERS_COUNT {
            multi_mesh.update_profile(i);

            global_renderer.draw_multi_mesh(multi_mesh, &mut self.materials.as_mut().unwrap()[i].borrow_mut(), i);
        }
    }

    pub fn stop_mesh_worker(&mut self) {
        self.mesh_gen_worker.stop();
    }

    pub fn process_mesh_gen(&mut self) {
        self.mesh_gen_worker.process_tasks();

        //while let Some(mesh_result) = self.mesh_gen_worker.get_finalized_task() {
        //    if let Some(old_mesh) = self.generated_mesh.remove(&mesh_result.chunk_pos) {
        //        self.restore_mesh_result(old_mesh);
        //    }

        //    self.generated_mesh.insert(mesh_result.chunk_pos, mesh_result);
        //}
    }

    pub fn clean_worker(&mut self) {
        self.mesh_gen_worker.clear();
    }

    pub fn get_generated_mesh(&mut self) -> Option<ChunkMeshResult> {
        self.mesh_gen_worker.get_finalized_task()
    }

    //pub fn get_generated_mesh(&mut self, chunk_pos: Vec3i) -> Option<ChunkMeshResult> {
    //    self.generated_mesh.remove(&chunk_pos)
    //}

    pub fn dispose_generated_mesh(&mut self, chunk_pos: Vec3i) {
        if let Some(mesh_result) = self.generated_mesh.remove(&chunk_pos) {
            self.restore_mesh_result(mesh_result);
        }
    }

    pub fn gen_mesh(&mut self,
        chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
        chunk_data: Arc<RwLock<ChunkData>>,
        chunk_pos: Vec3i
    ) {
        // SAFETY: blocks_manager reference is valid for all game time
        let blocks_manager = self.blocks_manager.clone();

        let vertices = array::from_fn(|_| self.chunk_mesh_vertices_pool.get_or(|| Vec::new()));
        let indices = array::from_fn(|_| self.chunk_mesh_indices_pool.get_or(|| Vec::new()));

        // create chunk mesh async
        self.mesh_gen_worker.add_task(move || {
            let mut mesh_result = ChunkMeshResult {
                neighbors_data: NeighborsChunksData::new_from_map(chunks_map, chunk_pos, true),
                chunk_data,

                vertices,
                indices,

                chunk_pos,
            };

            Chunk::gen_mesh(&mut mesh_result, &blocks_manager);
            mesh_result.gen_indices();

            return mesh_result;
        });
    }

    pub fn restore_mesh_result(&mut self, mesh_result: ChunkMeshResult) {
        for mut vertices in mesh_result.vertices {
            vertices.clear();
            self.chunk_mesh_vertices_pool.restore(vertices);
        }

        for mut indices in mesh_result.indices {
            indices.clear();
            self.chunk_mesh_indices_pool.restore(indices);
        }
    }
}
