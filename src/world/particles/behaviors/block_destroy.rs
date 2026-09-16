use rand::RngExt;

use crate::{
    math::{self, Vec2, Vec3},
    world::{
        Chunk, blocks::BlockIdState,
        particles::{
            ParticlesBehavior,
            ParticlesBehaviorStartArgs,
            ParticlesInfo
        }
    }
};


pub struct BlockDestroyBehavior;

impl BlockDestroyBehavior {
    pub fn start(args: &mut ParticlesBehaviorStartArgs, block_id_state: BlockIdState, block_pos: Vec3) {
        let block_properties = args.blocks_manager.get_properties(block_id_state);
        let world_tex_size = args.resources.world_texture.get_size();
        let particle_tex = block_properties.base_properties.model.particle_coords.denormalized(world_tex_size);
        let tex_size = args.resources.world_texture.get_atlas_tex_size(block_properties.base_properties.internal_name);

        const SCALE: f32 = 4.0;

        //let now = std::time::Instant::now();
        for _ in 0..20 {
            let mut p = ParticlesInfo::default();
            p.life = args.rand.random_range(0.4..1.5);
            p.size = Vec2::from1(args.rand.random_range(0.1..=0.2));

            let uv_offsetx = args.rand.random_range(0.0..=(tex_size.x - SCALE));
            let uv_offsety = args.rand.random_range(0.0..=(tex_size.y - SCALE));

            p.uv.minx = particle_tex.minx + uv_offsetx;
            p.uv.miny = particle_tex.miny + uv_offsety;
            p.uv.maxx = p.uv.minx + SCALE;
            p.uv.maxy = p.uv.miny + SCALE;
            p.uv = p.uv.normalized(world_tex_size);

            let rand_pos = Vec3::new(
                args.rand.random_range(0.0..=(1.0 - (p.size.x / 2.0))),
                args.rand.random_range(0.0..=(1.0 - (p.size.x / 2.0))),
                args.rand.random_range(0.0..=(1.0 - (p.size.x / 2.0))),
            );

            p.velocity = ((block_pos + rand_pos) - (block_pos + 0.5)).normalized() * 4.0;
            p.position = block_pos + rand_pos;

            args.create_particle(p);
        }
    }
}

impl ParticlesBehavior for BlockDestroyBehavior {
    fn update(&self, particle: &mut ParticlesInfo, dt: f32, chunk: Option<&Chunk>) {
        particle.update_velocity_x(ParticlesInfo::FRICTION, dt);
        particle.add_gravity(dt);
        particle.update_velocity_z(ParticlesInfo::FRICTION, dt);

        particle.update_position(dt);

        if particle.life < 0.3 {
            particle.size.x = math::lerp(0.0, particle.size.x, particle.life / 0.3);
            particle.size.y = math::lerp(0.0, particle.size.y, particle.life / 0.3);
        }
    }
}
