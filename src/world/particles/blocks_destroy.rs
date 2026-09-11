use crate::{math::{self, Vec3}, world::{Chunk, particles::ParticleFunc}};


pub struct BlockDestroy {}

impl ParticleFunc for BlockDestroy {
    fn start(&self, particle: &mut super::ParticleBase, pos: Vec3) {
        particle.position = pos;
    }

    fn update(&self, particle: &mut super::ParticleBase, dt: f32, chunk: Option<&Chunk>) {
        self.process_velocity(particle, dt);

        if particle.life < 0.3 {
            particle.size.x = math::lerp(0.0, particle.size.x, particle.life / 0.3);
            particle.size.y = math::lerp(0.0, particle.size.y, particle.life / 0.3);
        }
    }
}
