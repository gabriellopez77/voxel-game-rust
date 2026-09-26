use crate::{
    game::Directions, math::Vec3, resources::{ItemBlockModel, TexCoords}, world::{Aabb, blocks::{BlockIdState, BlockProperties, BlockTypes}},
};

pub struct PlaceBlockArgs {
    pub hit_normal: Vec3,

}

pub trait BlockBehaviors {
    fn get_properties(&self) -> &BlockProperties;

    fn get_type(&self) -> BlockTypes { BlockTypes::Default }

    fn is_opaque(&self) -> bool { return true }

    fn get_model(&self, state: u8) -> &ItemBlockModel { self.get_properties().get_model() }

    fn get_id_state(&self) -> BlockIdState { BlockIdState::new(self.get_properties().id, self.get_default_state()) }

    /// get item icon used by this block
    fn get_icon(&self, state: u8) -> TexCoords { self.get_model(state).icon_coords }

    /// get the default block state, used to represents this block
    fn get_default_state(&self) -> u8 { self.get_properties().default_state }

    /// indicates whether this block causes ambient occlusion
    fn causes_ambient_occlusion(&self) -> bool { return true }

    /// check if the ambient occlusion if applied on faces of this block
    fn affected_by_ambient_occlusion(&self) -> bool {
        let properties = self.get_properties();

        properties.get_model().ambient_occlusion && properties.light_emission == 0
    }

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

    fn place_block(&self, args: &PlaceBlockArgs) -> BlockIdState {
        BlockIdState::new(self.get_properties().id, self.get_default_state())
    }

    fn get_selection_box(&self, state: u8) -> Option<Aabb> { Some(Aabb::CUBE) }

    fn get_collision_box(&self, state: u8) -> Option<Aabb> { Some(Aabb::CUBE) }
}
