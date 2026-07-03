use std::{cell::RefCell,collections::HashMap,rc::Rc};
use crate::game::{Game, GameEvents};
use crate::inputs;
use crate::math::{Color4b, Matrix4, Vec2};
use crate::render::{GlobalRenderer, UiRenderer};
use crate::resources::ResourceManager;
use crate::ui::{ScreenResizeArgs, ScreenStartArgs};
use crate::ui::tools::Sprite;
use crate::ui::{ScreenBase, screen_base::ScreenInfo, ScreenUpdateArgs, screens::*};
use super::tools::UiElement;

type MutRc<T> = Rc<RefCell<T>>;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreensId {
    StartScreen,
    HudScreen,
    PauseScreen,
    LoadingScreen,
}

pub struct UiManager {
    ui_renderer: UiRenderer,

    pub projection: Matrix4,
    pub pixel_scale: f32,
    screen_size: Vec2,

    current_screen_id: ScreensId,
    screens: HashMap<ScreensId, MutRc<ScreenInfo>>,

    screens_background: Sprite,
    background_visible: bool,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            ui_renderer: UiRenderer::new(),

            projection: Matrix4::ZERO,
            pixel_scale: 3.0,
            screen_size: Vec2::ZERO,

            screens: HashMap::new(),
            current_screen_id: ScreensId::StartScreen,

            screens_background: Sprite::new(),
            background_visible: false,
        }
    }

    pub fn start(&mut self, resources: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        self.screens_background.set_texture(&resources.ui_sprites_texture, "white_color");

        self.screens_background.color = Color4b::new(0, 0, 0, 128);

        self.ui_renderer.start(global_renderer);

        self.add(ScreensId::StartScreen, StartScreen::new());
        self.add(ScreensId::HudScreen, HudScreen::new());
        self.add(ScreensId::PauseScreen, PauseScreen::new());
        self.add(ScreensId::LoadingScreen, LoadingScreen::new());
    }

    pub fn cleanup(&mut self) {
        self.ui_renderer.cleanup();
    }

    pub fn resize(&mut self, width: f32, height: f32, game: &Game) {
        // update pixel scale and screen size
        self.pixel_scale = 3.0;

        if width <= 1000.0 || height <= 750.0 { self.pixel_scale = 2.0 }
        if width >= 2200.0 || height >= 1200.0 { self.pixel_scale = 4.0 }
        if width >= 2800.0 || height >= 1800.0 { self.pixel_scale = 6.0 }

        self.screen_size = Vec2::new(width, height) / self.pixel_scale;

        self.projection = Matrix4::orthographic(0.0, width, height, 0.0);

        self.screens_background.set_sizev(self.screen_size + 1.0);

        let args = ScreenResizeArgs {
            screen_size: self.screen_size,
            screen_center: self.screen_size / 2.0,
            game
        };

        self.get_current_screen().borrow_mut().resize(&args);
    }

    pub fn update(&self, dt: f32, game: &mut Game) {
        let mut args = ScreenUpdateArgs {
            screen_size: self.screen_size,
            screen_center: self.screen_size / 2.0,

            mouse_pos: inputs::get_mouse_pos() / self.pixel_scale,

            game
        };

        self.get_current_screen().borrow_mut().update(dt, &mut args);
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if self.background_visible {
            self.screens_background.draw(&mut self.ui_renderer);
        }

        self.get_current_screen().borrow_mut().draw(&mut self.ui_renderer);

        self.ui_renderer.draw(global_renderer);
    }

    pub fn change(&mut self, id: ScreensId, game: &mut Game) {
        self.background_visible = false;

        if id == ScreensId::HudScreen {
            game.add_event(GameEvents::SetCursorMode(glfw::CursorMode::Disabled));
        }
        else {
            if game.is_in_world() {
                self.background_visible = true;
            }

            game.add_event(GameEvents::SetCursorMode(glfw::CursorMode::Normal));
        }

        let new_screen = self.screens[&id].clone();
        Self::change_logic(self.screen_size, &mut new_screen.borrow_mut(), &game.resources_manager, game);

        self.current_screen_id = id;
    }

    pub fn current_screen_is(&self, other: ScreensId) -> bool {
        self.current_screen_id == other
    }

    fn change_logic(screen_size: Vec2, screen_info: &mut ScreenInfo, resources: &ResourceManager, game: &Game) {
        let start_args = ScreenStartArgs {
            screen_size,
            screen_center: screen_size / 2.0,

            resources: resources,
            game
        };

        let resize_args = ScreenResizeArgs {
            screen_size,
            screen_center: screen_size / 2.0,

            game
        };

        let mut screen = screen_info.screen.borrow_mut();

        if !screen_info.started {
            screen_info.started = true;
            screen_info.screen_size = screen_size;
            screen_info.screen_center = screen_size / 2.0;
            screen.start(&start_args);

            // not resize if screen size in zero
            if screen_size != Vec2::ZERO {
                screen.resize(&resize_args);
            }
        }

        if screen_info.screen_size != screen_size {
            screen_info.screen_size = screen_size;
            screen_info.screen_center = screen_size / 2.0;
            screen.resize(&resize_args);
        }
    }

    pub fn add<T>(&mut self, id: ScreensId, screen: T)
    where
        T: ScreenBase,
        for<'a> T: 'a
    {
        let screen_info = ScreenInfo::new(Rc::new(RefCell::new(screen)));
        self.screens.insert(id, Rc::new(RefCell::new(screen_info)));
    }

    pub fn get_current_screen(&self) -> MutRc<dyn ScreenBase> {
        self.screens[&self.current_screen_id].borrow_mut().screen.clone()
    }
}
