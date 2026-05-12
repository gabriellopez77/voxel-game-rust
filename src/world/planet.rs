use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::math::Vec3i;
use crate::resources::ResourceManager;
use crate::world::chunk::Chunk;
use crate::world::WorldGen;

pub struct Planet {
    chunks: HashMap<Vec3i, Rc<RefCell<Chunk>>>,
    render_distance: i32,
    world_gen: WorldGen
}

impl Planet {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            render_distance: 8,
            world_gen: WorldGen::new()
        }
    }

    pub fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        let shader = resource_manager.borrow().get_shader("chunk").expect("shader not exists");
        //let texture = resource_manager.get_texture("chunk").expect("texture not exists");

        let pos = Vec3i::new(0, 0, 0);

        let chunk = Rc::new(RefCell::new(Chunk::new(pos, shader.clone())));

        self.world_gen.gen_data(pos, &mut chunk.borrow_mut().chunk_data);
        
        let (vertices, indices) = chunk.borrow_mut().gen_mesh();
        chunk.borrow_mut().renderer.create_vao(&vertices, &indices);

        self.chunks.insert(pos, chunk.clone());
    }

    pub fn draw(&mut self) {
        for (pos, chunk) in self.chunks.iter() {
            chunk.borrow_mut().renderer.draw();
        }
    }
}