use crate::render::material::MaterialType;
use crate::render::{GlobalRenderer, Material};
use crate::render::core::raw_buffer::BufferFlags;
use crate::world::chunk::ChunkMeshResult;


#[derive(Copy, Clone)]
pub enum RendererType {
    Opaque,
    Alpha,
}

impl RendererType {
    pub const RENDERS_COUNT: usize = 2;
}

pub struct ChunkRenderer {
    default_material: Material,
    water_material: Material,
}

impl ChunkRenderer {
    pub fn new(global_renderer: &mut GlobalRenderer) -> Self {
        Self {
            default_material: global_renderer.create_chunk_material(MaterialType::ChunksOpaque),
            water_material: global_renderer.create_chunk_material(MaterialType::ChunksAlpha),
        }
    }

    pub fn erase(&mut self) {
        self.default_material.destroy();
        self.water_material.destroy();
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        global_renderer.draw_obj(&self.default_material);
        global_renderer.draw_obj(&self.water_material);
    }

    pub fn update_mesh(&mut self, mesh_result: &ChunkMeshResult) {
        //let now = std::time::Instant::now();

        self.default_material.set_mesh(
            &mesh_result.vertices[RendererType::Opaque as usize],
            &mesh_result.indices[RendererType::Opaque as usize],
            BufferFlags::VRAM | BufferFlags::ONCE
        );

        self.water_material.set_mesh(
            &mesh_result.vertices[RendererType::Alpha as usize],
            &mesh_result.indices[RendererType::Alpha as usize],
            BufferFlags::VRAM | BufferFlags::ONCE
        );

        //println!("{}", now.elapsed().as_micros());
    }
}
