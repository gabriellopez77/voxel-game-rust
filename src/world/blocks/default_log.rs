use strum::IntoEnumIterator;

use crate::{
    math::Vec3,
    resources::{ItemBlockModel, item_block_model::{RotateAxis, RotateDegrees}},
    world::{
        blocks::{
            BlockBehaviors,
            BlockIdState,
            BlockProperties,
            block_behaviors::PlaceBlockArgs,
            block_states::{Axis, BlockStates, BlockStatesTypes},
        },
        items::{ItemCreation, ItemCreationArgs},
    }
};


pub struct DefaultLog {
    properties: BlockProperties,

    axis_x_model: ItemBlockModel,
    axis_z_model: ItemBlockModel,
}

impl BlockBehaviors for DefaultLog {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }

    fn get_model(&self, state: u8) -> &ItemBlockModel {
        let states = self.properties.get_states_from_idx(state);

        if states.has_state(BlockStatesTypes::Axis(Axis::X)) {
            return &self.axis_x_model
        }
        else if states.has_state(BlockStatesTypes::Axis(Axis::Z)) {
            return &self.axis_z_model
        }
        else {
            return self.properties.get_model()
        }
    }

    fn place_block(&self, args: &PlaceBlockArgs) -> BlockIdState {
        let axis = if args.hit_normal == Vec3::new(0.0, 0.0, -1.0) { Axis::Z }
        else if args.hit_normal == Vec3::new(0.0, 0.0, 1.0) { Axis::Z }
        else if args.hit_normal == Vec3::new(-1.0, 0.0, 0.0) { Axis::X }
        else if args.hit_normal == Vec3::new(1.0, 0.0, 0.0) { Axis::X }
        else { Axis::Y };

        return BlockIdState::new(
            self.properties.id,
            self.properties.get_states_idx(&[BlockStatesTypes::Axis(axis)]).unwrap() as u8
        );
    }
}

impl ItemCreation for DefaultLog {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        properties.can_replace = false;
        properties.light_filter = 15;
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        for axis in Axis::iter() {
            properties.add_states(BlockStates::new(vec![
                BlockStatesTypes::Axis(axis),
            ]));
        }

        properties.default_state = properties.get_states_idx(&[BlockStatesTypes::Axis(Axis::Y)]).unwrap() as u8;

        let center = Vec3::from1(8.0);
        let base_model = args.resources.get_model(args.internal_name);

        Self {
            properties: properties,

            axis_x_model: base_model.rotate_clone(center, RotateAxis::Z, RotateDegrees::Pos90),
            axis_z_model: base_model.rotate_clone(center, RotateAxis::X, RotateDegrees::Pos90),
        }
    }
}
