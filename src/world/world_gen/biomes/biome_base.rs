use crate::world::blocks::BlockProperties;


pub trait BiomeBase {
    fn get_surface_block(&self) -> &BlockProperties;
    fn get_underground_block(&self) -> &BlockProperties;
    fn get_surface_decorations(&self) -> &BlockProperties;
}
