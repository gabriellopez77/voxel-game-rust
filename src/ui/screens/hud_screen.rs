use std::{cell::RefCell, rc::Rc};

use crate::{math::Vec2, render::{SpritesRenderer, SpritesVertices, TextVertices}, resources::ResourceManager, ui::{ScreenBase, tools::{Sprite, UiElement}}};


pub struct HudScreen {
    screen_size: Vec2,
    screen_center: Vec2,
    started: bool,

    crosshair: Sprite,
}

impl ScreenBase for HudScreen {
    fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        self.crosshair.set_texture(resource_manager.borrow().get_texture("ui").unwrap().get_coords("crosshair"));
        self.crosshair.set_size(16.0, 16.0);
    }

    fn update(&mut self, dt: f32) {

    }

    fn draw(&mut self, sprite_renderer: &mut SpritesRenderer<SpritesVertices>, text_renderer: &mut SpritesRenderer<TextVertices>) {
        self.crosshair.draw(sprite_renderer);
    }

    fn resize(&mut self, screen_size: Vec2, screen_center: Vec2) {
        self.crosshair.set_posv(screen_center - self.crosshair.get_size());
    }

    fn change_logic(&mut self, screen_size: Vec2, resource_manager: Rc<RefCell<ResourceManager>>) {
        if !self.started {
            self.started = true;
            self.screen_size = screen_size;
            self.screen_center = screen_size / 2.0;
            self.start(resource_manager.clone());

            // not resize if screen size in zero
            if screen_size != Vec2::ZERO {
                self.resize(screen_size, self.screen_center);
            }
        }

        if self.screen_size != screen_size {
            self.screen_size = screen_size;
            self.screen_center = screen_size / 2.0;
            self.resize(screen_size, self.screen_center);
        }
    }
}

impl HudScreen {
    pub fn new() -> Self {
        Self {
            screen_size: Vec2::ZERO,
            screen_center: Vec2::ZERO,
            started: false,

            crosshair: Sprite::new(),
        }
    }
}
