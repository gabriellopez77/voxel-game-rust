use crate::{
    math::{self, Vec2, Vec3},
    world::Chunk,
    resources::TexCoords,
};


pub trait ParticlesBehavior {
    fn update(&self, particle: &mut ParticlesInfo, dt: f32, chunk: Option<&Chunk>);
}

#[derive(Default)]
pub struct ParticlesInfo {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: Vec2,
    pub uv: TexCoords,
    pub light_levels: u8,
    pub life: f32,
}

impl ParticlesInfo {
    pub const FRICTION: f32 = 6.0;
    pub const GRAVITY: f32 = 15.0;

    pub fn update_velocity_x(&mut self, friction: f32, dt: f32) {
        self.velocity.x -= self.velocity.x * (friction * dt);
    }

    pub fn update_velocity_y(&mut self, friction: f32, dt: f32) {
        self.velocity.y -= self.velocity.y * (friction * dt);
    }

    pub fn update_velocity_z(&mut self, friction: f32, dt: f32) {
        self.velocity.z -= self.velocity.z * (friction * dt);
    }

    pub fn add_gravity(&mut self, dt: f32) {
        self.velocity.y -= Self::GRAVITY * dt;
    }

    pub fn update_position(&mut self, dt: f32) {
        if self.velocity.x.abs() < math::EPSILON { self.velocity.x = 0.0 }
        if self.velocity.y.abs() < math::EPSILON { self.velocity.y = 0.0 }
        if self.velocity.z.abs() < math::EPSILON { self.velocity.z = 0.0 }

        self.position += self.velocity * dt;
    }
}
