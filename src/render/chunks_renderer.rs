use std::{cell::RefCell, rc::Rc};

use crate::{render::{ChunkVertices, GlobalRenderer, Material, MultiMesh, core::raw_buffer::BufferFlags, multi_mesh::MultiMeshInfo}, world::chunk::ChunkMeshResult};


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
}

impl ChunksRenderer {
    pub fn new() -> Self {
        Self {
            multi_mesh: None,

            materials: None
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let mut multi_mesh =  global_renderer.create_multi_mesh(size_of::<ChunkVertices>());
        multi_mesh.start(BufferFlags::VRAM | BufferFlags::RARE_UPDATE);
        multi_mesh.create_profile(BufferFlags::RAM);

        self.multi_mesh = Some(multi_mesh);

        self.materials = Some([
            global_renderer.get_material("chunks"),
            global_renderer.get_material("chunksAlpha"),
        ]);
    }

    pub fn cleanup(&mut self) {
        self.multi_mesh.as_mut().unwrap().destroy();
    }

    pub fn update_mesh(&mut self, info: &mut MultiMeshInfo, mesh_result: &ChunkMeshResult, render_type: ChunksRendererType) {
        let multi_mesh = self.multi_mesh.as_mut().unwrap();

        multi_mesh.remove_mesh(info);

        *info = multi_mesh.add_mesh(
            &mesh_result.vertices[render_type as usize],
            &mesh_result.indices[render_type as usize],
        );
    }

    pub fn destroy_mesh(&mut self, info: &mut MultiMeshInfo) {
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
}