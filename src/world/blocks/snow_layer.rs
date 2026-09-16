use crate::world::{blocks::{BlockBehaviors, BlockProperties, BlockTypes}, items::{ItemCreation, ItemCreationArgs}};


pub struct SnowLayer {
    properties: BlockProperties
}

impl BlockBehaviors for SnowLayer {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for SnowLayer {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = true;
        properties.is_transparent = true;
        properties.light_filter = 0;
        properties.block_type = BlockTypes::SnowLayer;
        properties.collision_box = None;
        properties.set_selection_box(0, 0, 0, 16, 2, 16);

        Self {
            properties: properties,
        }
    }
}
