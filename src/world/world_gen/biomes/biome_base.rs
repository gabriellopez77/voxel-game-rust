use crate::world::blocks::BlockBehaviors;


pub trait BiomeBase {
    fn get_surface_block(&self) -> &'static dyn BlockBehaviors;
    fn get_underground_block(&self) -> &'static dyn BlockBehaviors;
    fn get_surface_decorations(&self) -> &'static dyn BlockBehaviors;
}
