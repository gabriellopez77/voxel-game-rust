use std::collections::VecDeque;
use std::cell::RefCell;
use crate::game::{Game, GameEvents};
use crate::inputs::Inputs;
use crate::math::{Color4b, Matrix4, Vec2};
use crate::render::{GlobalRenderer, UiRenderer};
use crate::ui::screens::ui_common::UiCommonUpdateArgs;
use crate::ui::{ScreenResizeArgs, ScreenStartArgs};
use crate::ui::tools::Sprite;
use crate::ui::{ScreenBase, screen_base::ScreenInfo, ScreenUpdateArgs, screens::*};
use super::tools::UiElement;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreensId {
    StartScreen,
    HudScreen,
    PauseScreen,
    LoadingScreen,
    InventoryScreen,

    ScreensCount,
}

pub struct UiManager {
    ui_renderer: UiRenderer,

    pub projection: Matrix4,
    pub pixel_scale: f32,
    screen_size: Vec2,

    current_screen_id: ScreensId,
    ui_common: UiCommon,
    screens: [ScreenInfo; ScreensId::ScreensCount as usize],

    screens_background: Sprite,
    background_visible: bool,
    in_world: bool,
    first_change: bool,

    in_world_screens_stack: VecDeque<ScreensId>,
    out_world_screens_stack: VecDeque<ScreensId>,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            ui_renderer: UiRenderer::new(),

            projection: Matrix4::ZERO,
            pixel_scale: 3.0,
            screen_size: Vec2::ZERO,

            current_screen_id: ScreensId::StartScreen,
            ui_common: UiCommon::new(),
            screens: [
                Self::add(ScreensId::StartScreen, StartScreen::new()),
                Self::add(ScreensId::HudScreen, HudScreen::new()),
                Self::add(ScreensId::PauseScreen, PauseScreen::new()),
                Self::add(ScreensId::LoadingScreen, LoadingScreen::new()),
                Self::add(ScreensId::InventoryScreen, InventoryScreen::new()),
            ],

            screens_background: Sprite::new(),
            background_visible: false,
            in_world: false,
            first_change: true,

            in_world_screens_stack: VecDeque::new(),
            out_world_screens_stack: VecDeque::new(),
        }
    }

    pub fn start(&mut self, game: &mut Game) {
        self.screens_background.color = Color4b::new(0, 0, 0, 128);

        self.ui_renderer.start(&mut game.global_renderer);

        let start_args = ScreenStartArgs {
            resources: &game.resources_manager,
            game
        };

        self.ui_common.start(&start_args);
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

        self.screens[self.current_screen_id as usize].screen.borrow_mut().resize(&args);
        self.ui_common.resize(&args);
    }

    pub fn update(&mut self, dt: f32, game: &mut Game, inputs: &Inputs) {
        let mut args = ScreenUpdateArgs {
            dt,

            screen_size: self.screen_size,
            screen_center: self.screen_size / 2.0,

            mouse_pos: inputs.get_mouse_pos() / self.pixel_scale,

            game,
            inputs,
            ui_common: &mut self.ui_common,
        };

        self.screens[self.current_screen_id as usize].screen.borrow_mut().update(&mut args);

        let mut ui_common_args = UiCommonUpdateArgs {
            dt,

            screen_size: self.screen_size,
            screen_center: self.screen_size / 2.0,

            mouse_pos: inputs.get_mouse_pos() / self.pixel_scale,

            game,
            inputs,
        };
        self.ui_common.update(&mut ui_common_args);
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if self.background_visible {
            self.screens_background.draw(&mut self.ui_renderer);
        }

        self.screens[self.current_screen_id as usize].screen.borrow_mut().draw(&mut self.ui_renderer);
        self.ui_common.draw(&mut self.ui_renderer);

        self.ui_renderer.draw(global_renderer);
    }

    pub fn enter_world(&mut self, game: &mut Game) {
        self.in_world = true;

        // remove loading screen from stack
        self.out_world_screens_stack.pop_back().unwrap();
        self.out_world_screens_stack.clear();

        self.change(ScreensId::HudScreen, game);
    }

    pub fn leave_world(&mut self, game: &mut Game) {
        self.in_world = false;

        self.in_world_screens_stack.clear();

        self.change(ScreensId::StartScreen, game);
    }

    pub fn return_back(&mut self, game: &mut Game) {
        let new_screen_id: ScreensId;

        if self.in_world {
            // hud screen always is present in stack when we are in world, then change to pause screen and pause game
            if self.in_world_screens_stack.len() > 1 {
                self.in_world_screens_stack.pop_back().unwrap();
                new_screen_id = *self.in_world_screens_stack.back().unwrap();
            }
            else { return }
        }
        else {
            // start screen always is present in stack and we can not remove it
            if self.out_world_screens_stack.len() > 1 {
                self.out_world_screens_stack.pop_back().unwrap();
                new_screen_id = *self.out_world_screens_stack.back().unwrap();
            }
            else { return }
        }

        self.change_logic(self.screen_size, new_screen_id, game);
    }

    pub fn change(&mut self, id: ScreensId, game: &mut Game) {
        if self.current_screen_is(id) && !self.first_change { return }

        self.first_change = false;
        self.change_logic(self.screen_size, id, game);

        if self.in_world {
            self.in_world_screens_stack.push_back(id);
        }
        else {
            self.out_world_screens_stack.push_back(id);
        }
    }

    pub fn current_screen_is(&self, other: ScreensId) -> bool {
        self.current_screen_id == other
    }

    pub fn get_current_screen(&self) -> ScreensId {
        self.current_screen_id
    }

    fn change_logic(&mut self, screen_size: Vec2, new_screen_id: ScreensId, game: &mut Game) {
        game.world.player.inventory.clear_flying_item();

        let screen_info = &mut self.screens[new_screen_id as usize];

        self.background_visible = false;
        self.current_screen_id = screen_info.id;

        if screen_info.id == ScreensId::HudScreen {
            game.add_event(GameEvents::SetCursorMode(glfw::CursorMode::Disabled));
        }
        else {
            if self.in_world {
                self.background_visible = true;
            }

            game.add_event(GameEvents::SetCursorMode(glfw::CursorMode::Normal));
        }

        let start_args = ScreenStartArgs {
            resources: &game.resources_manager,
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

    pub fn add<T>(id: ScreensId, screen: T) -> ScreenInfo
    where
        T: ScreenBase,
        for<'a> T: 'a
    {
        return ScreenInfo::new(Box::new(RefCell::new(screen)), id);
    }
}
