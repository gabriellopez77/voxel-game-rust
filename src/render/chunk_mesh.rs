use crate::render::chunks_renderer::ChunksRendererType;
use crate::render::multi_mesh::MultiMeshInfo;
use crate::render::ChunksRenderer;
use crate::world::chunk::ChunkMeshResult;




pub struct ChunkMesh {
    default_mesh: MultiMeshInfo,
    water_mesh: MultiMeshInfo,

    //fade_in_effect: f32,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
            default_mesh: MultiMeshInfo::new(),
            water_mesh: MultiMeshInfo::new(),

            //fade_in_effect: 0.0,
        }
    }

    pub fn erase(&mut self, renderer: &mut ChunksRenderer) {
        renderer.destroy_mesh(&mut self.default_mesh);
        renderer.destroy_mesh(&mut self.water_mesh);
    }

    pub fn draw(&mut self, _: f32, renderer: &mut ChunksRenderer) {
        //if self.fade_in_effect < 0.95 {
        //    self.fade_in_effect = math::lerp(self.fade_in_effect, 1.0, dt * 4.0);
        //}
        //else {
        //    self.fade_in_effect = 1.0;
        //}

        renderer.record_draw(self.default_mesh, ChunksRendererType::Opaque);
        renderer.record_draw(self.water_mesh, ChunksRendererType::Alpha);
    }

    pub fn update_mesh(&mut self, mesh_result: &ChunkMeshResult, renderer: &mut ChunksRenderer) {
        //let now = std::time::Instant::now();

        renderer.update_mesh(&mut self.default_mesh, mesh_result, ChunksRendererType::Opaque);
        renderer.update_mesh(&mut self.water_mesh, mesh_result, ChunksRendererType::Alpha);
    
        //println!("{}", now.elapsed().as_micros());
    }
}
