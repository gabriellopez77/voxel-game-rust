use std::collections::VecDeque;
use std::{rc::Rc, cell::RefCell};
use crate::render::{GlobalRenderer, core::VulkanApp};
use crate::ui::ui_manager::ScreensId;
use crate::world::World;
use crate::inputs::{self, Inputs};
use crate::resources::ResourceManager;
use crate::ui::UiManager;
use crate::window::Window;
use crate::world::world::WorldUpdateArgs;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStates {
    Loading,
    Playable,
    Saving,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerStates {
    Active,
    Menu,
}

pub enum GameEvents {
    QuitGame,
    SetCursorMode(glfw::CursorMode),
    ChangeScreen(ScreensId),
    LoadChunks,
    EnterToWorld,
    LeaveToWorld,
    PauseGame,
    UnpauseGame,
}

pub struct Game {
    pub resources_manager: ResourceManager,
    pub global_renderer: GlobalRenderer,

    pub world: World,

    ui_manager: Rc<RefCell<UiManager>>,

    state: GameStates,
    paused: bool,
    in_world: bool,

    events_queue: VecDeque<GameEvents>,

    //pub imgui_renderer: Option<imgui_rs_vulkan_renderer::Renderer>,
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

            //imgui_renderer: None,
        }
    }

    pub fn start(&mut self, app: &mut VulkanApp, imgui: &mut imgui::Context) {
        self.resources_manager.start(app);
        self.global_renderer.start(&mut self.resources_manager);

        self.ui_manager.clone().borrow_mut().start(self);
        self.add_event(GameEvents::ChangeScreen(ScreensId::StartScreen));

        self.world.start(&mut self.resources_manager, &mut self.global_renderer);

        //let now = std::time::Instant::now();
        //println!("{}", now.elapsed().as_micros());

        //self.imgui_renderer = Some(imgui_rs_vulkan_renderer::Renderer::with_default_allocator(
        //    &app.ash_instance,
        //    app.physical_device,
        //    app.ash_device.clone(),
        //    app.graphics_queue,
        //    app.graphics_command_pool,
        //    app.render_pass,
        //    imgui,
        //    Some(imgui_rs_vulkan_renderer::Options {
        //        in_flight_frames: 2,
        //        ..Default::default()
        //    })
        //).unwrap());

        //imgui.io_mut().display_size = [1050.0, 650.0];
    }

    pub fn update(&mut self, dt: f32, window: &mut Window, inputs: &mut Inputs) {
        // process events
        self.process_events(window);


        if inputs.key_pressed(inputs::Keys::Escape) {
            if self.in_world {
                if self.ui_manager.borrow().current_screen_is(ScreensId::HudScreen) {
                    self.add_event(GameEvents::ChangeScreen(ScreensId::PauseScreen));
                    self.add_event(GameEvents::PauseGame);
                }
                else {
                    if self.ui_manager.borrow().current_screen_is(ScreensId::PauseScreen) {
                        self.add_event(GameEvents::UnpauseGame);
                    }

                    self.ui_manager.clone().borrow_mut().return_back(self);
                }
            }
            else {
                self.ui_manager.clone().borrow_mut().return_back(self);
            }
        }
        if inputs.key_pressed(inputs::Keys::F3) {
            self.ui_manager.borrow_mut().toggle_debug_screen_visibily();
        }


        if self.state == GameStates::Loading {
            self.world.planet.process_chunks_gen();

            if self.world.planet.pendings_chunks_count == 0 {
                self.add_event(GameEvents::EnterToWorld);
            }
        }


        if self.in_world {
            let mut update_args = WorldUpdateArgs {
                is_paused: self.paused,
                events_queue: &mut self.events_queue,
                inputs,
                dt,
                current_screen_id: self.ui_manager.borrow().get_current_screen(),
                resources: &mut self.resources_manager,
            };

            self.world.update(dt, &mut update_args);
        }

        self.ui_manager.clone().borrow_mut().update(dt, self, inputs);
    }

    pub fn render(&mut self, dt: f32, imgui: &mut imgui::Context) {
        self.global_renderer.begin();

        if self.in_world {
            self.world.draw(dt, &mut self.global_renderer);
        }

        self.ui_manager.borrow_mut().draw(&mut self.global_renderer);

        // update ubo
        let ubo = &mut self.global_renderer.global_ubo;

        // ui
        ubo.data.ui_proj.0 = self.ui_manager.borrow().projection;
        ubo.data.ui_pixel_scale = self.ui_manager.borrow().pixel_scale;
        ubo.flush_all_data();

        self.global_renderer.end();

        // 1. Build the ImGui user interface
        //let ui = imgui.new_frame();

        //if let Some(wt) = ui.window("Debug Menu")
        //    .size([700.0, 500.0], imgui::Condition::FirstUseEver)
        //    .begin() {

        //    let first_person = &mut self.world.player.first_person;

        //    //ui.slider("Position X", -5.0, 5.0, &mut first_person.interact_position.x);
        //    //ui.slider("Position Y", -5.0, 5.0, &mut first_person.interact_position.y);
        //    //ui.slider("Position Z", -5.0, 5.0, &mut first_person.interact_position.z);

        //    //ui.slider("Scale X", 0.0, 1.0, &mut first_person.hand_scale.x);
        //    //ui.slider("Scale Y", 0.0, 1.0, &mut first_person.hand_scale.y);
        //    //ui.slider("Scale Z", 0.0, 1.0, &mut first_person.hand_scale.z);

        //    //ui.slider("Rotate X", -360.0, 360.0, &mut first_person.interact_rotation.x);
        //    //ui.slider("Rotate Y", -360.0, 360.0, &mut first_person.interact_rotation.y);
        //    //ui.slider("Rotate Z", -360.0, 360.0, &mut first_person.interact_rotation.z);

        //    wt.end();
        //}

        //self.imgui_renderer.as_mut().unwrap()
        //    .cmd_draw(self.global_renderer.app.get_graphics_cmd(), &imgui.render())
        //    .expect("Failed to record ImGui draw commands");
    }

    pub fn cleanup(&mut self, app: &mut VulkanApp) {
        self.world.cleanup();

        self.ui_manager.borrow_mut().cleanup();
        self.resources_manager.cleanup(app);
        self.global_renderer.cleanup();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.ui_manager.clone().borrow_mut().resize(width, height, self);
        self.world.player.camera.resize(width, height);
    }

    pub fn is_in_world(&self) -> bool { self.in_world }
    pub fn is_paused(&self) -> bool { self.paused }

    pub fn add_event(&mut self, event: GameEvents) {
        self.events_queue.push_back(event);
    }

    fn process_events(&mut self, window: &mut Window) {
        while let Some(event) = self.events_queue.pop_front() {
            match event {
                GameEvents::QuitGame => window.close(),
                GameEvents::SetCursorMode(mode) => window.set_cursor(mode),
                GameEvents::ChangeScreen(id) => self.ui_manager.clone().borrow_mut().change(id, self),
                GameEvents::LoadChunks => {
                    self.add_event(GameEvents::ChangeScreen(ScreensId::LoadingScreen));
                    self.state = GameStates::Loading;
                    self.world.load();
                }
                GameEvents::EnterToWorld => {
                    self.in_world = true;
                    self.paused = false;
                    self.state = GameStates::Playable;

                    self.ui_manager.clone().borrow_mut().enter_world(self);
                }
                GameEvents::LeaveToWorld => {
                    self.in_world = false;
                    self.world.leave();

                    self.ui_manager.clone().borrow_mut().leave_world(self);
                }
                GameEvents::PauseGame => { self.paused = true; }
                GameEvents::UnpauseGame => { self.paused = false; }
            }
        }
    }
}
