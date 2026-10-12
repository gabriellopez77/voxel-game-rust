use crate::{math, math::Vec3, render::{EntitiesRenderer}, resources::ResourcesManager, world::{
    Aabb,
    Chunk,
    Planet,
    particles::ParticlesManager
}};


pub enum EntityTypes {
    Player,
    ItemDrop,
}

pub trait EntityBase {
    fn get_properties(&mut self) -> &mut EntityProperties;
    fn get_type(&self) -> EntityTypes;

    fn start(&mut self, resources: &mut ResourcesManager);
    fn update(&mut self, args: &mut EntityUpdateArgs);
    fn draw(&mut self, renderer: &mut EntitiesRenderer);
}

pub struct EntityUpdateArgs<'a> {
    pub dt: f32,
    pub chunk: Option<&'a Chunk>,
    pub planet: &'a Planet,
    pub particles_manager: &'a mut ParticlesManager,
    pub cubes_vec: &'a mut Vec<Aabb>,
}

#[derive(Default)]
pub struct EntityProperties {
    pub aabb: Aabb,
    pub velocity: Vec3,

    pub light_level: u8,
    pub on_ground: bool,

    pub need_removed: bool,
}

impl EntityProperties {
    pub fn apply_gravity(&mut self, dt: f32) {
        const GRAVITY: f32 = 9.83 * 2.0;

        self.velocity.y -= GRAVITY * dt;
    }

    pub fn process_collision(&mut self, friction: f32, args: &mut EntityUpdateArgs) {
        self.velocity.x -= self.velocity.x * (friction * args.dt);
        if self.velocity.y > 0.0 { self.velocity.y -= self.velocity.y * (friction * args.dt) }
        self.velocity.z -= self.velocity.z * (friction * args.dt);


        if self.velocity.x.abs() < math::EPSILON { self.velocity.x = 0.0 }
        if self.velocity.y.abs() < math::EPSILON { self.velocity.y = 0.0 }
        if self.velocity.z.abs() < math::EPSILON { self.velocity.z = 0.0 }

        let mut velo = self.velocity * args.dt;
        let velo_org = velo;

        args.planet.get_blocks_hitboxes(&self.aabb.expandv(velo), args.cubes_vec);
        self.aabb.response_collision(&mut velo, args.cubes_vec);

        self.on_ground = velo_org.y != velo.y && velo_org.y < 0.0;

        if velo_org.x != velo.x { self.velocity.x = 0.0 }
        if velo_org.y != velo.y { self.velocity.y = 0.0 }
        if velo_org.z != velo.z { self.velocity.z = 0.0 }
    }
}
