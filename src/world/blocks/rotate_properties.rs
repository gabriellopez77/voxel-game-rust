use crate::{
    math::Vec3,
    resources::{ItemBlockModel, item_block_model::{RotateAxis, RotateDegrees}},
    world::{
        Aabb,
        blocks::block_states::{BlockStates, BlockStatesTypes, Facing}
    }
};


pub struct RotateProperties {
    north_model: ItemBlockModel,
    south_model: ItemBlockModel,
    west_model: ItemBlockModel,
    east_model: ItemBlockModel,

    north_selection_box: Aabb,
    south_selection_box: Aabb,
    west_selection_box: Aabb,
    east_selection_box: Aabb,

    north_collision_box: Aabb,
    south_collision_box: Aabb,
    west_collision_box: Aabb,
    east_collision_box: Aabb,
}

impl RotateProperties {
    pub fn new(
        base_model: &ItemBlockModel,
        model_axis: RotateAxis,
        base_selection_box: Aabb,
        selection_box_axis: RotateAxis,
        base_collision_box: Aabb
    ) -> Self {
        let center = Vec3::from1(8.0);

        Self {
            north_model: base_model.rotate_clone(center, model_axis, RotateDegrees::Neg90),
            south_model: base_model.rotate_clone(center, model_axis, RotateDegrees::Pos90),
            west_model: base_model.rotate_clone(center, model_axis, RotateDegrees::Pos180),
            east_model: base_model.clone(),

            north_selection_box: base_selection_box.clone_rotate(center, selection_box_axis, RotateDegrees::Neg90),
            south_selection_box: base_selection_box.clone_rotate(center,selection_box_axis, RotateDegrees::Pos90),
            west_selection_box: base_selection_box.clone_rotate(center, selection_box_axis, RotateDegrees::Pos180),
            east_selection_box: base_selection_box,

            north_collision_box: base_selection_box.clone_rotate(center, RotateAxis::X, RotateDegrees::Neg90),
            south_collision_box: base_selection_box.clone_rotate(center, RotateAxis::X, RotateDegrees::Pos90),
            west_collision_box: base_selection_box.clone_rotate(center, RotateAxis::X, RotateDegrees::Pos180),
            east_collision_box: base_collision_box,

        }
    }

    pub fn get_model<'a>(&'a self, states: &BlockStates, default_value: &'a ItemBlockModel) -> &'a ItemBlockModel {
        if states.has_state(BlockStatesTypes::Facing(Facing::North)) {
            return &self.north_model;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::South)) {
            return &self.south_model;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::West)) {
            return& self.west_model;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::East)) {
            return& self.east_model;
        }
        else { default_value }
    }

    pub fn get_selection_box(&self, states: &BlockStates, default_value: Aabb) -> Aabb {
        if states.has_state(BlockStatesTypes::Facing(Facing::North)) {
            return self.north_selection_box;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::South)) {
            return self.south_selection_box;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::West)) {
            return self.west_selection_box;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::East)) {
            return self.east_selection_box;
        }
        else { return default_value }
    }

    pub fn get_collision_box(&self, states: &BlockStates, default_value: Aabb) -> Aabb {
        if states.has_state(BlockStatesTypes::Facing(Facing::North)) {
            return self.north_collision_box;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::South)) {
            return self.south_collision_box;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::West)) {
            return self.west_collision_box;
        }
        else if states.has_state(BlockStatesTypes::Facing(Facing::East)) {
            return self.east_collision_box;
        }
        else { return default_value}
    }
}
