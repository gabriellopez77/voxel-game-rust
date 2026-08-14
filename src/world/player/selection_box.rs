use crate::math::{Vec3, Vec4};
use crate::render::material::MaterialType;
use crate::render::{GlobalRenderer, Material, Mesh, OUTLINE_CUBE_INDICES, OUTLINE_CUBE_VERTICES};
use crate::render::core::raw_buffer::BufferFlags;
use crate::world::player::RaycastingResult;


pub struct SelectionBox {
    renderer: Option<(Mesh, Material)>,

    position: Vec3,
    size: Vec3,

    visible: bool,
}

impl SelectionBox {
    pub fn new() -> Self {
        Self {
            renderer: None,

            position: Vec3::ZERO,
            size: Vec3::ZERO,

            visible: false,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_material("selectionBox", MaterialType::Alpha);
        mesh.set(&OUTLINE_CUBE_VERTICES, &OUTLINE_CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);

        self.renderer = Some((mesh, material));
    }

    pub fn cleanup(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        renderer.0.destroy();
        renderer.1.destroy();
    }

    pub fn update(&mut self, result: &Option<RaycastingResult>) {
        if let Some(result) = result {
            self.position = result.block_selection_box.get_min();
            self.size = result.block_selection_box.get_size();
            self.visible = true;
        }
        else {
            self.visible = false;
        }
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if !self.visible { return }

        let renderer = self.renderer.as_mut ().unwrap();
        global_renderer.set_push_constant(0, &self.position);
        global_renderer.set_push_constant(size_of::<Vec4>(), &self.size);
        global_renderer.draw(&renderer.0, &mut renderer.1);

    }
}
