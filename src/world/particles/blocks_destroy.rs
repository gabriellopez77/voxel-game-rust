use crate::{math::{Vec2, Vec3}, world::particles::ParticleFunc};


pub struct BlockDestroy {}

impl ParticleFunc for BlockDestroy {
    fn start(&self, particle: &mut super::ParticleBase, pos: Vec3) {
        particle.position = pos;
    }

    fn update(&self, particle: &mut super::ParticleBase, dt: f32) {
        self.process_velocity(particle, dt);
    }
}
