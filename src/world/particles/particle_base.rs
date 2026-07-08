use crate::{math::{self, Vec2, Vec3}, resources::TexCoords};


pub trait ParticleFunc {
    fn start(&self, particle: &mut ParticleBase, pos: Vec3);
    fn update(&self, particle: &mut ParticleBase, dt: f32);

    fn process_velocity(&self, particle: &mut ParticleBase, dt: f32) {
        pub const FRICTION: f32 = 6.0;
        const GRAVITY: f32 = 15.0;

        particle.velocity.x -= particle.velocity.x * (FRICTION * dt);
        particle.velocity.y -= GRAVITY * dt;
        particle.velocity.z -= particle.velocity.z * (FRICTION * dt);

        // epsilon
        if particle.velocity.x.abs() < math::EPSILON { particle.velocity.x = 0.0 }
        if particle.velocity.y.abs() < math::EPSILON { particle.velocity.y = 0.0 }
        if particle.velocity.z.abs() < math::EPSILON { particle.velocity.z = 0.0 }

        particle.position += particle.velocity * dt;
    }

    fn dead_animation(&self, particle: &mut ParticleBase, factor: f32) {
        particle.size.x = math::lerp(particle.size.x, 0.0, factor * 8.0);
        particle.size.y = math::lerp(particle.size.y, 0.0, factor * 8.0);
    }
}

pub struct ParticleBase {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: Vec2,
    pub uv: TexCoords,
    pub life: f32,
}

impl ParticleBase {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            size: Vec2::ZERO,
            uv: TexCoords::ZERO,
            life: 0.0,
        }
    }
}
