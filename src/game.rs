use std::collections::VecDeque;
use std::{rc::Rc, cell::RefCell};
use crate::render::{GlobalRenderer, VulkanApp};
use crate::ui::ui_manager::ScreensId;
use crate::world::World;
use crate::inputs;
use crate::resources::ResourceManager;
use crate::ui::UiManager;
use crate::window::Window;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStates {
    Loading,
    Playable,
    Saving,
}

pub enum GameEvents {
    QuitGame,
    SetCursorMode(glfw::CursorMode),
    ChangeScreen(ScreensId),
    LoadChunks,
    EnterToWorld,
    LeaveToWorld,
}

pub struct Game {
    pub resources_manager: ResourceManager,
    global_renderer: GlobalRenderer,

    pub world: World,

    ui_manager: Rc<RefCell<UiManager>>,

    state: GameStates,
    paused: bool,
    in_world: bool,

    events_queue: VecDeque<GameEvents>,
}

impl Game {
    pub fn new(app: &mut VulkanApp) -> Self {
        Self {
            resources_manager: ResourceManager::new(),
            global_renderer: GlobalRenderer::new(app),
            world: World::new(),

            ui_manager: Rc::new(RefCell::new(UiManager::new())),

            state: GameStates::Playable,
            paused: false,
            in_world: false,

            events_queue: VecDeque::new(),
        }
    }

    pub fn start(&mut self, app: &mut VulkanApp) {
        self.resources_manager.start(app, &mut self.global_renderer);
        self.global_renderer.start(&self.resources_manager);

        self.ui_manager.clone().borrow_mut().start(&self.resources_manager, &mut self.global_renderer);
        self.add_event(GameEvents::ChangeScreen(ScreensId::StartScreen));

        self.world.start(&self.resources_manager, &mut self.global_renderer);

        //let now = std::time::Instant::now();
        //println!("{}", now.elapsed().as_micros());
    }

    pub fn update(&mut self, dt: f32, window: &mut Window) {
        if inputs::key_pressed(inputs::Keys::Escape) && self.in_world {
            self.paused = !self.paused;

            if self.ui_manager.borrow().current_screen_is(ScreensId::HudScreen) {
                self.paused = true;
                self.ui_manager.clone().borrow_mut().change(ScreensId::PauseScreen, self);
            }
            else {
                self.paused = false;
                self.ui_manager.clone().borrow_mut().change(ScreensId::HudScreen, self);
            }
        }

        if self.state == GameStates::Loading {
            self.world.planet.process_chunks_gen();

            if self.world.planet.pendings_chunks_count == 0 {
                self.add_event(GameEvents::EnterToWorld);
            }
        }

        if self.in_world && !self.paused {
            self.world.update(dt);
        }

        self.ui_manager.clone().borrow_mut().update(dt, self);

        // process events
        while let Some(event) = self.events_queue.pop_front() {
            match event {
                GameEvents::EnterToWorld => self.enter_in_world(),
                GameEvents::QuitGame => window.close(),
                GameEvents::SetCursorMode(mode) => window.set_cursor(mode),
                GameEvents::ChangeScreen(id) => self.ui_manager.clone().borrow_mut().change(id, self),
                GameEvents::LoadChunks => {
                    self.add_event(GameEvents::ChangeScreen(ScreensId::LoadingScreen));
                    self.state = GameStates::Loading;
                    self.world.load();
                },
                GameEvents::LeaveToWorld => {
                    self.in_world = false;
                    self.world.leave();
                    self.add_event(GameEvents::ChangeScreen(ScreensId::StartScreen));
                },
            }
        }
    }

    pub fn render(&mut self) {
        self.global_renderer.begin();

        // update ubo
        let ubo = &mut self.resources_manager.global_ubo;

        // ui
        ubo.update("uiProj", self.ui_manager.borrow().projection.as_ptr());
        ubo.update("uiPixelScale", &self.ui_manager.borrow().pixel_scale);



        if self.in_world {
            self.world.draw(ubo, &mut self.global_renderer);
        }

        self.ui_manager.borrow_mut().draw(&mut self.global_renderer);

        self.global_renderer.end();
    }

    pub fn cleanup(&mut self, app: &mut VulkanApp) {
        self.world.cleanup();

        self.ui_manager.borrow_mut().cleanup();
        self.resources_manager.cleanup(app);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.ui_manager.clone().borrow_mut().resize(width, height, self);
        self.world.player.camera.resize(width, height);
    }

    pub fn is_in_world(&self) -> bool { self.in_world }
    pub fn is_paused(&self) -> bool { self.paused }

    fn enter_in_world(&mut self) {
        self.in_world = true;
        self.state = GameStates::Playable;

        self.add_event(GameEvents::ChangeScreen(ScreensId::HudScreen));
    }

    pub fn add_event(&mut self, event: GameEvents) {
        self.events_queue.push_back(event);
    }
}
