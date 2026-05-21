use std::{cell::RefCell, rc::Rc};
use crate::render::{SpritesRenderer, SpritesVertices, TextVertices};
use crate::resources::ResourceManager;


pub trait ScreenBase {
    fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>);
    fn update(&mut self, dt: f32);
    fn draw(&mut self, sprite_renderer: &mut SpritesRenderer<SpritesVertices>, text_renderer: &mut SpritesRenderer<TextVertices>);
    fn resize(&mut self, width: f32, height: f32);

    fn change_logic(&mut self, width: f32, height: f32, resource_manager: Rc<RefCell<ResourceManager>>);
}