use crate::world::blocks::BlockIdState;


pub trait BiomeBase {
    fn get_surface_block(&self) -> BlockIdState;
    fn get_underground_block(&self) -> BlockIdState;
    fn get_surface_decorations(&self) -> BlockIdState;
}
