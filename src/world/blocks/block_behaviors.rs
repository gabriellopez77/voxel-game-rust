use crate::{
    game::Directions, world::{
        blocks::{BlockIdState, BlockProperties, BlockTypes},
        items::ItemBaseProperties
    },
};


pub trait BlockBehaviors {
    fn get_properties(&self, state: u8) -> &BlockProperties;

    fn get_standard_properties(&self) -> &BlockProperties {
        self.get_properties(0)
    }

    fn get_base(&self) -> &ItemBaseProperties { &self.get_properties(0).base_properties }

    fn get_id_state(&self) -> BlockIdState { self.get_base().get_id_state() }

    fn get_type(&self) -> BlockTypes { BlockTypes::Default }

    fn is_opaque(&self) -> bool { return true }

    /// indicates whether this block causes ambient occlusion
    fn causes_ambient_occlusion(&self) -> bool { return true }

    /// check if the ambient occlusion if applied on faces of this block
    fn affected_by_ambient_occlusion(&self) -> bool { self.get_properties(0).base_properties.model.ambient_occlusion }

    /// used to set a custom light level for this block
    fn compute_light_levels(&self, levels: u8) -> u8 { return levels }

    /// used to enable or disable block face shade
    fn compute_face_shade(&self, shade: bool) -> bool { return shade }

    /// check if current block face should be renderered if around block is equals to it
    fn should_render_face_twin(&self, dir: Directions) -> bool { return true }

    /// check if current block face should be renderered
    fn should_render_face(&self, dir: Directions, around: &dyn BlockBehaviors) -> bool {
        let around_type = around.get_type();

        if self.is_opaque() {
            if dir == Directions::Up {
                if around_type == BlockTypes::SnowLayer || around_type == BlockTypes::Slab {
                    return false
                }
            }
        }
        else {
            if around.is_opaque() {
                return false;
            }
        }

        return true;
    }
}
