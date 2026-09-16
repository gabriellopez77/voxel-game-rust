use std::{cell::RefCell, rc::Rc};

use rand::rngs::ThreadRng;

use crate::{
    math::{Vec2, Vec3, math},
    render::{
        GlobalRenderer,
        Material,
        Mesh,
        vertices_data::{
            PARTICLES_VERTICES,
            SPRITES_INDICES,
            ParticlesVertices,
        },
        core::raw_buffer::{BufferFlags, BufferResizeMode}
    },
    resources::ResourceManager,
    world::{
        Chunk,
        Planet,
        blocks::{BlockIdState, BlocksManager},
        chunk::{ChunkGetter, chunk_data::ChunkDataReadBehavior},
        light_engine::{self, LightType},
        particles::{
            behaviors::{
                BlockDestroyBehavior,
            },
            ParticlesBehavior,
            ParticlesInfo}
    }
};


pub enum ParticlesSpawnArgs {
    BlockDestroy(BlockIdState, Vec3),
}

pub struct ParticlesBehaviorStartArgs<'a> {
    pub resources: &'a ResourceManager,
    pub blocks_manager: &'a BlocksManager,
    pub rand: &'a mut ThreadRng,

    behavior: Rc<dyn ParticlesBehavior>,
    particles_info: &'a mut Vec<(Rc<dyn ParticlesBehavior>, ParticlesInfo)>,
}

impl<'a> ParticlesBehaviorStartArgs<'a> {
    pub fn create_particle(&mut self, particle: ParticlesInfo) {
        self.particles_info.push((self.behavior.clone(), particle));
    }
}

pub struct ParticlesManager {
    renderer: Option<(Mesh, Rc<RefCell<Material>>)>,
    instance_data: Vec<ParticlesVertices>,

    destroy_behavior: Rc<dyn ParticlesBehavior>,

    particles_info: Vec<(Rc<dyn ParticlesBehavior>, ParticlesInfo)>,

    spawn_list: Vec<ParticlesSpawnArgs>,

    rand: ThreadRng,
}

impl ParticlesManager {
    pub const MAX_PARTICLES_COUNT: usize = 1000;

    pub fn new() -> Self {
        Self {
            renderer: None,
            instance_data: Vec::new(),

            destroy_behavior: Rc::new(BlockDestroyBehavior{}),

            particles_info: Vec::new(),

            spawn_list: Vec::new(),

            rand: rand::rng(),
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_and_get_material("particles");
        mesh.set(&PARTICLES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<ParticlesVertices>() * Self::MAX_PARTICLES_COUNT, None, BufferFlags::RAM);

        self.renderer = Some((mesh, material));
    }

    pub fn update(&mut self,
        dt: f32,
        resources: &ResourceManager,
        blocks_manager: &BlocksManager,
        planet: &Planet
    ) {
        //let now = std::time::Instant::now();

        self.process_particles_spawn(resources, blocks_manager);

        // sort particles by chunk position
        self.particles_info.sort_by_key(|p| {
            let chunk_pos = math::get_chunk_pos(p.1.position);

            (chunk_pos.x, chunk_pos.y, chunk_pos.z)
        });
        //println!("sort time: {}us", now.elapsed().as_micros());

        let mut chunk_getter = ChunkGetter::new();

        //let now = std::time::Instant::now();
        for i in (0..self.particles_info.len()).rev() {
            let (behavior, particle) = &mut self.particles_info[i];
            let mut ch: Option<&Chunk> = None;
            let chunk_pos = math::get_chunk_pos(particle.position);

            chunk_getter.change(chunk_pos, &planet.chunks_manager);

            if let Some(ref chunk) = chunk_getter.chunk {
                let chunk_block = math::get_chunk_block(chunk_pos, particle.position);

                particle.light_levels = chunk.data.get_light(chunk_block, LightType::Both);

                ch = Some(&chunk);
            }
            else {
                particle.light_levels = light_engine::MAX_SKY_LEVEL;
            }


            behavior.update(particle, dt, ch);

            particle.life -= dt;
            if particle.life < 0.0 {
                self.particles_info.swap_remove(i);
            }
        }

        //println!("update time: {}us", now.elapsed().as_micros());
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer, camera_rotate: Vec2) {
        let rot = Vec3::new(0.0, -camera_rotate.x.to_radians(), camera_rotate.y.to_radians());

        for (_, particle) in &self.particles_info {
            self.instance_data.push(ParticlesVertices {
                position: particle.position,
                scale: Vec3::new(0.0, particle.size.y, particle.size.x),
                rotation: rot,
                uv: particle.uv,
                light_levels: particle.light_levels,
                texture_idx: GlobalRenderer::WORLD_TEXTURE_IDX,
            })
        }

        let renderer = self.renderer.as_mut().unwrap();
        global_renderer.draw_instanced_with_buffer(
            &mut renderer.0,
            &mut renderer.1.borrow_mut(),
            &mut self.instance_data,
            BufferResizeMode::Discard
        );
    }

    pub fn reset(&mut self) {
        self.particles_info.clear();
    }

    pub fn spawn(&mut self, args: ParticlesSpawnArgs) {
        self.spawn_list.push(args);
    }

    fn process_particles_spawn(&mut self, resources: &ResourceManager, blocks_manager: &BlocksManager) {
        let mut start_args = ParticlesBehaviorStartArgs {
            resources,
            blocks_manager,
            rand: &mut self.rand,
            behavior: self.destroy_behavior.clone(),
            particles_info: &mut self.particles_info,
        };

        while let Some(args) = self.spawn_list.pop() {
            match args {
                ParticlesSpawnArgs::BlockDestroy(block_id_state, block_pos) => {
                    BlockDestroyBehavior::start(&mut start_args, block_id_state, block_pos);
                }
            }
        }
    }
}
