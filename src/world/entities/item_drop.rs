use std::{cell::RefCell, rc::Rc};

use rand::RngExt;

use crate::{
    math::{Matrix4, Vec3},
    render::{EntitiesRenderer, Mesh},
    resources::ResourcesManager,
    world::{
        blocks::{BlockIdState, block_registry},
        entities::{EntityBase, EntityProperties, EntityTypes, EntityUpdateArgs},
    }
};
use crate::world::Aabb;

pub struct ItemDrop {
    pub properties: EntityProperties,

    life: f32,

    id_state: BlockIdState,
    model: Option<Rc<RefCell<Mesh>>>,
}

impl EntityBase for ItemDrop {
    fn get_properties(&mut self) -> &mut EntityProperties { &mut self.properties }

    fn get_type(&self) -> EntityTypes { EntityTypes::ItemDrop }

    fn start(&mut self, resources: &mut ResourcesManager) {
        let model = block_registry::get().get(self.id_state).get_model(self.id_state.state);
        let internal_name = block_registry::get().get_properties(self.id_state).internal_name;

        self.model = Some(resources.get_or_load_model_mesh(internal_name, model));
    }

    fn update(&mut self, args: &mut EntityUpdateArgs) {
        self.life += args.dt;

        if self.life > 10.0 {
            self.properties.need_removed = true;
        }

        self.properties.apply_gravity(args.dt);
        self.properties.process_collision(3.0, args);
    }

    fn draw(&mut self, renderer: &mut EntitiesRenderer) {
        let mut pos = self.properties.aabb.get_min();
        pos.y += (self.life * 2.0).cos() * 0.1 + 0.15;

        let mut model = Matrix4::IDENTITY;
        model.translatev(pos);

        model.translatev(Vec3::from1(0.35) * 0.5);
        model.rotate_y(self.life * 60.0);
        model.translatev(Vec3::from1(0.35) * -0.5);

        model.scalev(Vec3::from1(0.35));

        renderer.add_item_model(model, self.properties.light_level, self.model.clone().unwrap());
    }
}

impl ItemDrop {
    pub fn new(position: Vec3, id_state: BlockIdState) -> Self {
        let mut properties = EntityProperties::default();
        properties.aabb = Aabb::new(0.0, 0.0, 0.0, 0.25, 0.25, 0.25).clone_movev(position - 0.25 / 2.0);

        Self {
            properties,

            life: 0.0,

            id_state,
            model: None,
        }
    }

    pub fn new_from_block_break(global_block: Vec3, id_state: BlockIdState) -> Self {
        let mut item_drop = Self::new(global_block + 0.5, id_state);
        item_drop.properties.velocity = Vec3::new(
            rand::rng().random_range(-1.0..=1.0),
            rand::rng().random_range(0.1..=1.0),
            rand::rng().random_range(-1.0..=1.0),
        ) * rand::rng().random_range(0.0..=7.0);

        return item_drop;
    }

}
