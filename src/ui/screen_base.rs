use crate::render::SpritesRenderer;


pub trait ScreenBase {
    fn start(&mut self);
    fn update(&mut self, dt: f32);
    fn draw(&mut self, renderer: &mut SpritesRenderer);
    fn resize(&mut self, width: f32, height: f32);

    fn change_logic(&mut self, width: f32, height: f32);
}