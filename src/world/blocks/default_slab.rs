use std::sync::Arc;

use strum::IntoEnumIterator;

use crate::{
    game::Directions,
    math::Vec3,
    resources::ItemBlockModel,
    world::{
        Aabb,
        blocks::{
            BlockBehaviors,
            BlockIdState,
            BlockProperties,
            BlockTypes,
            block_behaviors::{BlockBehaviorsState, PlaceBlockArgs},
            block_states::{BlockStates, BlockStatesTypes, SlabType}
        },
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct DefaultSlab {
    properties: BlockProperties,

    top_model: Arc<ItemBlockModel>,
    bottom_model: Arc<ItemBlockModel>,
}

impl BlockBehaviors for DefaultSlab {
    fn get_properties(&self) -> &BlockProperties { &self.properties }
    fn get_type(&self) -> BlockTypes { BlockTypes::Slab }
    fn causes_ambient_occlusion(&self) -> bool { return false }
    fn is_opaque(&self) -> bool { return false }
    fn should_render_face_twin(&self, state: u8, dir: Directions, other: BlockBehaviorsState) -> bool {
        dir.is_vertical() || state != other.1
    }

    fn should_render_face(&self, face: Directions, around: &dyn BlockBehaviors) -> bool {
        return !(face != Directions::Up && around.is_opaque());
    }

    fn get_collision_box(&self, state: u8) -> Option<Aabb> {
        let states = self.properties.get_states_from_idx(state);

        if states.has_state(BlockStatesTypes::SlabType(SlabType::Bottom)) {
            return Some(Aabb::new_cube(0, 0, 0, 16, 8, 16));
        }

        return Some(Aabb::new_cube(0, 8, 0, 16, 8, 16));
    }

    fn get_selection_box(&self, state: u8) -> Option<Aabb> {
        let states = self.properties.get_states_from_idx(state);

        if states.has_state(BlockStatesTypes::SlabType(SlabType::Bottom)) {
            return Some(Aabb::new_cube(0, 0, 0, 16, 8, 16));
        }

        return Some(Aabb::new_cube(0, 8, 0, 16, 8, 16));
    }

    fn get_model(&self, state: u8) -> &ItemBlockModel {
        let states = self.properties.get_states_from_idx(state);

        if states.has_state(BlockStatesTypes::SlabType(SlabType::Bottom)) {
            return &self.bottom_model;
        }

        return &self.top_model;
    }

    fn place_block(&self, args: &PlaceBlockArgs) -> BlockIdState {
        let slab_type = if args.hit_normal == Vec3::new(0.0, -1.0, 0.0) { SlabType::Top }
        else { SlabType::Bottom };

        return BlockIdState::new(
            self.properties.id,
            self.properties.get_states_idx(&[BlockStatesTypes::SlabType(slab_type)]).unwrap() as u8
        );
    }
}

impl ItemCreation for DefaultSlab {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_filter = 0;

        for axis in SlabType::iter() {
            properties.add_states(BlockStates::new(vec![
                BlockStatesTypes::SlabType(axis),
            ]));
        }

        Self {
            properties: properties,

            top_model: args.resources.get_model(&format!("{}_top", args.internal_name)),
            bottom_model: args.resources.get_model(args.internal_name),
        }
    }
}
