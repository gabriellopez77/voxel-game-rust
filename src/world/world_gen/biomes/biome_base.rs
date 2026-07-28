pub trait BiomeBase {
    fn get_surface_block(&self) -> (u16, u8);
    fn get_underground_block(&self) -> (u16, u8);
    fn get_surface_decorations(&self) -> (u16, u8);
}
