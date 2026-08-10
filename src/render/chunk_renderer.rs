use crate::math;
use crate::render::material::MaterialType;
use crate::render::{GlobalRenderer, Material, Mesh};
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
    default_mesh: Mesh,

    water_material: Material,
    water_mesh: Mesh,

    fade_in_effect: f32,
}

impl ChunkRenderer {
    pub fn new(global_renderer: &mut GlobalRenderer) -> Self {
        Self {
            default_material: global_renderer.create_chunk_material(MaterialType::ChunksOpaque),
            default_mesh: global_renderer.create_mesh(),

            water_material: global_renderer.create_chunk_material(MaterialType::ChunksAlpha),
            water_mesh: global_renderer.create_mesh(),

            fade_in_effect: 0.0,
        }
    }

    pub fn erase(&mut self) {
        self.default_material.destroy();
        self.default_mesh.destroy();

        self.water_material.destroy();
        self.water_mesh.destroy();
    }

    pub fn draw(&mut self, dt: f32, global_renderer: &mut GlobalRenderer) {
        if self.default_mesh.get_triangles_count() == 0 && self.water_mesh.get_triangles_count() == 0 {
            return
        }

        if self.fade_in_effect < 0.95 {
            self.fade_in_effect = math::lerp(self.fade_in_effect, 1.0, dt * 4.0);
        }
        else {
            self.fade_in_effect = 1.0;
        }


        global_renderer.set_push_constant(0, &self.fade_in_effect);
        global_renderer.draw(&self.default_mesh, &self.default_material);

        global_renderer.set_push_constant(0, &self.fade_in_effect);
        global_renderer.draw(&self.water_mesh, &self.water_material);
    }

    pub fn update_mesh(&mut self, mesh_result: &ChunkMeshResult) {
        //let now = std::time::Instant::now();

        if !mesh_result.vertices[RendererType::Opaque as usize].is_empty() {
            self.default_mesh.set(
                &mesh_result.vertices[RendererType::Opaque as usize],
                &mesh_result.indices[RendererType::Opaque as usize],
                BufferFlags::VRAM | BufferFlags::ONCE
            );
        }

        if !mesh_result.vertices[RendererType::Alpha as usize].is_empty() {
            self.water_mesh.set(
                &mesh_result.vertices[RendererType::Alpha as usize],
                &mesh_result.indices[RendererType::Alpha as usize],
                BufferFlags::VRAM | BufferFlags::ONCE
            );
        }

        //println!("{}", now.elapsed().as_micros());
    }
}
