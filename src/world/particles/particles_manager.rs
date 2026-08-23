use std::{cell::RefCell, rc::Rc};

use rand::{RngExt, rngs::ThreadRng};

use crate::{math::{Vec2, Vec3}, render::{GlobalRenderer, Material, Mesh, PARTICLES_VERTICES, ParticlesVertices, SPRITES_INDICES, core::raw_buffer::{BufferFlags, BufferResizeMode}}, resources::ResourceManager, utils::NullSafePtr, world::{blocks::BlockProperties, particles::{BlockDestroy, ParticleBase, ParticleFunc}}};


struct ParticlesInfo {
    func: Rc<dyn ParticleFunc>,
    particle: ParticleBase,
    dead_delay: f32,
}

pub enum ParticlesSpawnArgs<'a> {
    BlockDestroy(&'a BlockProperties, Vec3),
}

pub struct ParticlesManager {
    renderer: Option<(Mesh, Rc<RefCell<Material>>)>,
    instance_data: Vec<ParticlesVertices>,
    resources: NullSafePtr<ResourceManager>,

    destroy_func: Rc<dyn ParticleFunc>,

    particles_info: Vec<ParticlesInfo>,

    rand: ThreadRng,
}

impl ParticlesManager {
    pub const MAX_PARTICLES_COUNT: usize = 1000;

    pub fn new() -> Self {
        Self {
            renderer: None,
            instance_data: Vec::new(),
            resources: NullSafePtr::null(),

            destroy_func: Rc::new(BlockDestroy{}),
            particles_info: Vec::new(),

            rand: rand::rng(),
        }
    }

    pub fn start(&mut self, resources_manager: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_and_get_material("particles");
        mesh.set(&PARTICLES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<ParticlesVertices>() * Self::MAX_PARTICLES_COUNT, None, BufferFlags::RAM);

        self.renderer = Some((mesh, material));
        self.resources = NullSafePtr::new(resources_manager);
    }

    pub fn update(&mut self, dt: f32) {
        for i in (0..self.particles_info.len()).rev() {
            let info = &mut self.particles_info[i];
            let p = &mut info.particle;

            p.life -= dt;
            info.func.update(p, dt);

            if p.life < 0.0 {
                self.destroy_func.dead_animation(p, dt);

                info.dead_delay -= dt;
            }

            if info.dead_delay < 0.0 {
                self.particles_info.swap_remove(i);
            }
        }
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer, camera_rotate: Vec2) {
        let rot = Vec3::new(0.0, -camera_rotate.x.to_radians(), camera_rotate.y.to_radians());

        let len = self.particles_info.len().min(Self::MAX_PARTICLES_COUNT);

        for i in 0..len {
            let p = &self.particles_info[i].particle;

            self.instance_data.push(ParticlesVertices {
                position: p.position,
                scale: Vec3::new(0.0, p.size.y, p.size.x),
                rotation: rot,
                uv: p.uv,
                texture_idx: GlobalRenderer::WORLD_TEXTURE_IDX,
            })
        }

        let renderer = self.renderer.as_mut().unwrap();
        global_renderer.draw_instanced_with_buffer(&mut renderer.0, &mut renderer.1.borrow_mut(), &mut self.instance_data, BufferResizeMode::Discard);
    }

    pub fn spawn(&mut self, args: ParticlesSpawnArgs) {
        match args {
            ParticlesSpawnArgs::BlockDestroy(block_properties, block_pos) => {
                let particle_tex = block_properties.base_properties.model.particle_coords.denormalized(self.resources.world_texture.get_size());

                const SCALE: f32 = 4.0;

                for _ in 0..20 {
                    let mut p = ParticleBase::new();
                    p.life = self.rand.random_range(0.4..1.5);
                    p.size = Vec2::from1(self.rand.random_range(0.1..=0.2));

                    let tex_size = self.resources.world_texture.get_atlas_tex_size(block_properties.base_properties.internal_name);

                    let offsetx = self.rand.random_range(0.0..=(tex_size.x - SCALE));
                    let offsety = self.rand.random_range(0.0..=(tex_size.y - SCALE));

                    p.uv.minx = particle_tex.minx + offsetx;
                    p.uv.miny = particle_tex.miny + offsety;
                    p.uv.maxx = p.uv.minx + SCALE;
                    p.uv.maxy = p.uv.miny + SCALE;
                    p.uv = p.uv.normalized(self.resources.world_texture.get_size());

                    let func = self.destroy_func.clone();
                    let rand_pos = Vec3::new(
                        self.rand.random_range(0.0..=(1.0 - (p.size.x / 2.0))),
                        self.rand.random_range(0.0..=(1.0 - (p.size.x / 2.0))),
                        self.rand.random_range(0.0..=(1.0 - (p.size.x / 2.0))),
                    );

                    p.velocity = ((block_pos + rand_pos) - (block_pos + 0.5)).normalized() * 4.0;

                    func.start(&mut p, block_pos + rand_pos);
                    self.add(func, p);
                }
            }
        }
    }

    pub fn cleanup(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        renderer.0.destroy();
    }

    pub fn reset(&mut self) {
        self.particles_info.clear();
    }

    fn add(&mut self, func: Rc<dyn ParticleFunc>, p: ParticleBase) {
        self.particles_info.push(ParticlesInfo { func, particle: p, dead_delay: 1.0 });
    }
}
