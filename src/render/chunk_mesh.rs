use crate::math;
use crate::render::chunks_renderer::{ChunkInstanceData, ChunkMeshResult, ChunkRendererType};
use crate::render::multi_mesh::MultiMeshInfo;
use crate::render::ChunksRenderer;




pub struct ChunkMesh {
    default_mesh: MultiMeshInfo,
    water_mesh: MultiMeshInfo,

    instance_data: ChunkInstanceData,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
            default_mesh: MultiMeshInfo::new(),
            water_mesh: MultiMeshInfo::new(),

            instance_data: ChunkInstanceData::default(),
        }
    }

    pub fn dispose(&mut self, renderer: &mut ChunksRenderer) {
        renderer.dispose_mesh(&mut self.default_mesh);
        renderer.dispose_mesh(&mut self.water_mesh);
    }

    pub fn draw(&mut self, dt: f32, renderer: &mut ChunksRenderer) {
        if self.instance_data.fade_in_effect < 0.95 {
            self.instance_data.fade_in_effect = math::lerp(self.instance_data.fade_in_effect, 1.0, dt * 4.0);
        }
        else {
            self.instance_data.fade_in_effect = 1.0;
        }
        //self.instance_data.fade_in_effect = 0.1;

        renderer.record_draw(self.default_mesh, self.instance_data, ChunkRendererType::Opaque);
        renderer.record_draw(self.water_mesh, self.instance_data, ChunkRendererType::Alpha);
    }

    pub fn update_mesh(&mut self, mesh_result: &ChunkMeshResult, renderer: &mut ChunksRenderer) {
        //let now = std::time::Instant::now();

        renderer.update_mesh(&mut self.default_mesh, mesh_result, ChunkRendererType::Opaque);
        renderer.update_mesh(&mut self.water_mesh, mesh_result, ChunkRendererType::Alpha);

        //println!("{}", now.elapsed().as_micros());
    }
}
