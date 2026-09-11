use crate::world::Chunk;


pub trait EntityBase {
    fn start();
    fn tick(chunk: Option<&Chunk>);
    fn render();
    fn on_collider();
}
