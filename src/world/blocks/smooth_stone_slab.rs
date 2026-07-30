use crate::world::{Aabb, blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}};


pub struct SmoothStoneSlab {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for SmoothStoneSlab {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for SmoothStoneSlab {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.block_type = super::BlockTypes::Slab;
        properties.can_replace = false;
        properties.is_transparent = true;
        properties.light_filter = 0;
        properties.collision_box = Some(Aabb::new(0.0, 0.0, 0.0, 1.0, 0.5, 1.0));
        properties.set_selection_box(0, 0, 0, 16, 8, 16);

        Self {
            properties: vec![properties],
        }
    }
}
