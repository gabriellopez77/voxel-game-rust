use crate::math::{Vec3, Vec4};
use crate::render::material::MaterialType;
use crate::render::{GlobalRenderer, Material, OUTLINE_CUBE_INDICES, OUTLINE_CUBE_VERTICES};
use crate::render::core::raw_buffer::BufferFlags;
use crate::world::player::RaycastingResult;


pub struct SelectionBox {
    material: Option<Material>,

    position: Vec3,
    size: Vec3,

    visible: bool,
}

impl SelectionBox {
    pub fn new() -> Self {
        Self {
            material: None,

            position: Vec3::ZERO,
            size: Vec3::ZERO,

            visible: false,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let mut material = global_renderer.create_material("selectionBox", MaterialType::Alpha);
        material.set_mesh(&OUTLINE_CUBE_VERTICES, &OUTLINE_CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);


        self.material = Some(material);
    }

    pub fn cleanup(&mut self) {
        self.material.as_mut().unwrap().destroy();
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

        let material = self.material.as_mut().unwrap();
        material.update_push_constant(0, &self.position);
        material.update_push_constant(size_of::<Vec4>(), &self.size);
        global_renderer.draw_obj(material);

    }
}
