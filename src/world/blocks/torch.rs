use strum::IntoEnumIterator;

use crate::{
    math::Vec3, resources::{ItemBlockModel, item_block_model::RotateAxis}, world::{
        Aabb,
        blocks::{
            BlockBehaviors,
            BlockIdState,
            BlockProperties,
            block_behaviors::PlaceBlockArgs,
            block_states::{BlockStates, BlockStatesTypes, Facing::self},
            rotate_properties::RotateProperties
        },
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct Torch {
    properties: BlockProperties,
    rotate_properties: RotateProperties,
}

impl BlockBehaviors for Torch {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }

    fn is_opaque(&self) -> bool { false }

    fn causes_ambient_occlusion(&self) -> bool { false }

    fn get_collision_box(&self, state: u8) -> Option<Aabb> { None }

    fn get_selection_box(&self, state: u8) -> Option<Aabb> {
        let states = self.properties.get_states_from_idx(state);

        Some(self.rotate_properties.get_selection_box(&states, Aabb::new_cube(6, 0, 6, 4, 11, 4)))
    }

    fn get_model(&self, state: u8) -> &ItemBlockModel {
        let states = self.properties.get_states_from_idx(state);

        self.rotate_properties.get_model(&states, self.properties.get_model())
    }

    fn place_block(&self, args: &PlaceBlockArgs) -> BlockIdState {
        let facing = if args.hit_normal == Vec3::new(0.0, 0.0, -1.0) { Facing::North }
        else if args.hit_normal == Vec3::new(0.0, 0.0, 1.0) { Facing::South }
        else if args.hit_normal == Vec3::new(-1.0, 0.0, 0.0) { Facing::West }
        else if args.hit_normal == Vec3::new(1.0, 0.0, 0.0) { Facing::East }
        else { Facing::Up };

        return BlockIdState::new(
            self.properties.id,
            self.properties.get_states_idx(&[BlockStatesTypes::Facing(facing)]).unwrap() as u8
        );
    }
}

impl ItemCreation for Torch {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_emission = 14;
        properties.light_filter = 0;


        for facing in Facing::iter() {
            properties.add_states(BlockStates::new(vec![
                BlockStatesTypes::Facing(facing),
            ]));
        }


        Self {
            properties: properties,
            rotate_properties: RotateProperties::new(
                &args.resources.get_model("torch_wall"), RotateAxis::Y,
                Aabb::new_cube(0, 2, 6, 5, 11, 4), RotateAxis::Y,
                Aabb::CUBE
            ),
        }
    }
}
