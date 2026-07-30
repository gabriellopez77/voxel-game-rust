use crate::math;
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

    fade_in_effect: f32,
}

impl ChunkRenderer {
    pub fn new(global_renderer: &mut GlobalRenderer) -> Self {
        Self {
            default_material: global_renderer.create_chunk_material(MaterialType::ChunksOpaque),
            water_material: global_renderer.create_chunk_material(MaterialType::ChunksAlpha),

            fade_in_effect: 0.0,
        }
    }

    pub fn erase(&mut self) {
        self.default_material.destroy();
        self.water_material.destroy();
    }

    pub fn draw(&mut self, dt: f32, global_renderer: &mut GlobalRenderer) {
        if !self.default_material.is_suitable_to_draw() && !self.water_material.is_suitable_to_draw() {
            return
        }

        if self.fade_in_effect < 0.95 {
            self.fade_in_effect = math::lerp(self.fade_in_effect, 1.0, dt * 4.0);
        }
        else {
            self.fade_in_effect = 1.0;
        }

        self.default_material.update_push_constant(0, &self.fade_in_effect);
        self.water_material.update_push_constant(0, &self.fade_in_effect);


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
