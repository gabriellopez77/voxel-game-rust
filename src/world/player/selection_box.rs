use crate::math::Vec3;
use crate::render::material::MaterialType;
use crate::render::{CUBE_INDICES, CUBE_VERTICES, GlobalRenderer, Material};
use crate::render::raw_buffer::BufferFlags;


pub struct SelectionBox {
    material: Option<Material>,

    position: Vec3,

    visible: bool,
}

impl SelectionBox {
    pub fn new() -> Self {
        Self {
            material: None,

            position: Vec3::ZERO,

            visible: false,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let mut material = global_renderer.create_material("selectionBox", MaterialType::Alpha);
        material.set_mesh(&CUBE_VERTICES, &CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);

        self.material = Some(material);
    }

    pub fn cleanup(&mut self) {
        self.material.as_mut().unwrap().destroy();
    }

    pub fn update(&mut self, dt: f32, position: Option<Vec3>) {
        if let Some(position) = position {
            self.position = position;
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
        global_renderer.draw_obj(material);

    }
}
