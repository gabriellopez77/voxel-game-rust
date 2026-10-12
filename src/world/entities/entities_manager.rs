use crate::{
    math,
    render::{EntitiesRenderer, GlobalRenderer},
    resources::ResourcesManager,
    world::{
        Planet,
        ChunksManager,
        chunk::chunk_data::ChunkDataReadBehavior,
        entities::{EntityBase, EntityUpdateArgs},
        light_engine::{self, LightType},
        particles::ParticlesManager
    }
};
use crate::world::Aabb;

pub struct EntitiesManager {
    pub renderer: EntitiesRenderer,

    add_queue: Vec<Box<dyn EntityBase>>,

    non_chunk_entities: Vec<Box<dyn EntityBase>>,

    cubes_vec: Vec<Aabb>,
}

impl EntitiesManager {
    pub fn new() -> Self {
        Self {
            renderer: EntitiesRenderer::new(),

            add_queue: Vec::new(),

            non_chunk_entities: Vec::new(),

            cubes_vec: Vec::new(),
        }
    }

    pub fn start(&mut self, global_renderer: &GlobalRenderer) {
        self.renderer.start(global_renderer);
    }

    pub fn update(&mut self,
        dt: f32,
        resources: &mut ResourcesManager,
        planet: &mut Planet,
        particles_manager: &mut ParticlesManager,
    ) {
        let chunks = planet.chunks_manager.get_ordered_chunks_list();

        while let Some(mut entity) = self.add_queue.pop() {
            let chunk_pos = math::get_chunk_pos(entity.get_properties().aabb.get_min());

            if let Some(chunk) = planet.chunks_manager.get_chunk(chunk_pos) {
                entity.start(resources);

                chunk.content.borrow_mut().entities.push(entity);
            }
        }

        for chunk in chunks {
            let mut content = chunk.content.borrow_mut();

            for i in (0..content.entities.len()).rev() {
                if content.entities[i].get_properties().need_removed {
                    content.entities.swap_remove(i);

                    continue;
                }


                let last_chunk_pos = math::get_chunk_pos(content.entities[i].get_properties().aabb.get_min());

                let mut args = EntityUpdateArgs {
                    dt,
                    chunk: Some(chunk),
                    planet,
                    particles_manager,
                    cubes_vec: &mut self.cubes_vec,
                };

                content.entities[i].update(&mut args);

                let global_pos = content.entities[i].get_properties().aabb.get_min();
                let new_chunk_pos = math::get_chunk_pos(global_pos);
                let chunk_block = math::get_chunk_block(new_chunk_pos, global_pos);

                content.entities[i].get_properties().light_level = chunk.data.get_light(chunk_block, LightType::Both);

                // the entity has changed of chunk
                if last_chunk_pos != new_chunk_pos {
                    if let Some(chunk) = planet.chunks_manager.get_chunk(new_chunk_pos) {
                        chunk.content.borrow_mut().entities.push(content.entities.swap_remove(i));
                    }
                    else {
                        self.non_chunk_entities.push(content.entities.swap_remove(i));
                    }
                }
            }
        }


        for i in (0..self.non_chunk_entities.len()).rev() {
            if self.non_chunk_entities[i].get_properties().need_removed {
                self.non_chunk_entities.swap_remove(i);

                continue;
            }

            let last_chunk_pos = math::get_chunk_pos(self.non_chunk_entities[i].get_properties().aabb.get_min());

            let mut args = EntityUpdateArgs {
                dt,
                chunk: None,
                planet,
                particles_manager,
                cubes_vec: &mut self.cubes_vec,
            };

            self.non_chunk_entities[i].update(&mut args);
            self.non_chunk_entities[i].get_properties().light_level = light_engine::MAX_BOTH_LEVEL;

            let new_chunk_pos = math::get_chunk_pos(self.non_chunk_entities[i].get_properties().aabb.get_min());

            // the entity has changed of chunk
            if last_chunk_pos != new_chunk_pos {
                if let Some(chunk) = planet.chunks_manager.get_chunk(new_chunk_pos) {
                    chunk.content.borrow_mut().entities.push(self.non_chunk_entities.swap_remove(i));
                }
            }
        }
    }

    pub fn draw(&mut self,
        chunks_manager: &ChunksManager,
        global_renderer: &mut GlobalRenderer
    ) {
        let chunks = chunks_manager.get_ordered_chunks_list();

        for chunk in chunks {
            let mut ch = chunk.content.borrow_mut();

            if !ch.inside_frustum { continue }

            for entity in &mut ch.entities {
                entity.draw(&mut self.renderer);
            }
        }

        for entity in &mut self.non_chunk_entities {
            entity.draw(&mut self.renderer);
        }
        
        self.renderer.draw(global_renderer);
    }

    pub fn add_entity<T>(&mut self, entity: T)
    where
        T: EntityBase, for<'a> T: 'a
    {
        self.add_queue.push(Box::new(entity));
    }
}
