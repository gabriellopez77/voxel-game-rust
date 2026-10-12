use std::{cell::RefCell, rc::Rc};

use crate::{
    math::Matrix4, render::{
        GlobalRenderer, Material, Mesh,
        core::raw_buffer::{BufferFlags, BufferResizeMode},
        vertices_data::{
            CUBE_INDICES, ENTITIES_CUBES_VERTICES, EntitiesCubesVertices
        }
    }
};

pub struct EntitiesRenderer {
    renderer: Option<(Mesh, Rc<RefCell<Material>>)>,

    instance_data: Vec<EntitiesCubesVertices>,

    item_model_material: Option<Rc<RefCell<Material>>>,
    item_model_draw_list: Vec<(Matrix4, u8, Rc<RefCell<Mesh>>)>,
}

impl EntitiesRenderer {
    pub fn new() -> Self {
        Self {
            renderer: None,

            instance_data: Vec::new(),

            item_model_material: None,
            item_model_draw_list: Vec::new(),
        }
    }

    pub fn start(&mut self, global_renderer: &GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_and_get_material("entities");

        mesh.set(&ENTITIES_CUBES_VERTICES, &CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<EntitiesCubesVertices>() * 64, None, BufferFlags::VRAM);

        self.item_model_material = Some(global_renderer.get_material("itemDrop"));
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

        let mut item_model_material = self.item_model_material.as_mut().unwrap().borrow_mut();

        for (matrix, light_level, mesh) in &self.item_model_draw_list {
            let light_level = *light_level as u32;

            global_renderer.set_push_constant(0, matrix);
            global_renderer.set_push_constant(Matrix4::SIZEOF, &light_level);
            global_renderer.draw(&mesh.borrow(), &mut item_model_material);
        }

        self.item_model_draw_list.clear();
    }

    pub fn add_item_model(&mut self, matrix: Matrix4, light_level: u8, mesh: Rc<RefCell<Mesh>>) {
        self.item_model_draw_list.push((matrix, light_level, mesh));
    }

    pub fn add_cube(&mut self, cube: EntitiesCubesVertices) {
        self.instance_data.push(cube);
    }
}
