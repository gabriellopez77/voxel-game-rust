use std::{cell::RefCell, rc::Rc};

use crate::render::{CUBE_INDICES, ENTITIES_CUBES_VERTICES, EntitiesCubesVertices, GlobalRenderer, Material, Mesh, core::raw_buffer::{BufferFlags, BufferResizeMode}};


pub struct EntitiesRenderer {
    renderer: Option<(Mesh, Rc<RefCell<Material>>)>,

    instance_data: Vec<EntitiesCubesVertices>,
}

impl EntitiesRenderer {
    pub fn new() -> Self {
        Self {
            renderer: None,

            instance_data: Vec::new(),
        }
    }

    pub fn start(&mut self, global_renderer: &GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_and_get_material("entities");

        mesh.set(&ENTITIES_CUBES_VERTICES, &CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<EntitiesCubesVertices>() * 64, None, BufferFlags::VRAM);

        self.renderer = Some((mesh, material));
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        let sprites_renderer = self.renderer.as_mut().unwrap();
        global_renderer.draw_instanced_with_buffer(
            &mut sprites_renderer.0,
            &mut sprites_renderer.1.borrow_mut(),
            &mut self.instance_data,
            BufferResizeMode::Discard
        );
    }

    pub fn cleanup(&mut self) {
        self.renderer.as_mut().unwrap().0.destroy();
    }

    pub fn add_cube(&mut self, cube: EntitiesCubesVertices) {
        self.instance_data.push(cube);
    }
}
