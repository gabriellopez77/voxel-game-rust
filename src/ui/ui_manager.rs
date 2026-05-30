use std::{
    any::TypeId,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::math::Vec2;
use crate::render::UiRenderer;
use crate::resources::ResourceManager;
use crate::ui::{ScreenBase, screen_base::ScreenInfo, StartScreen, HudScreen};

type MutRc<T> = Rc<RefCell<T>>;


pub struct UiManager {
    ui_renderer: UiRenderer,

    resource_manager: Option<Rc<RefCell<ResourceManager>>>,

    pixel_scale: f32,
    screen_size: Vec2,

    current_screen: Option<MutRc<ScreenInfo>>,
    screens: HashMap<TypeId, MutRc<ScreenInfo>>,
}

impl UiManager {
    pub fn new() -> Self {
        Self {
            ui_renderer: UiRenderer::new(),

            resource_manager: None,

            pixel_scale: 3.0,
            screen_size: Vec2::ZERO,

            screens: HashMap::new(),
            current_screen: None,
        }
    }

    pub fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        self.ui_renderer.start(&resource_manager.borrow());
        self.resource_manager = Some(resource_manager.clone());

        self.add(HudScreen::new());
        self.add(StartScreen::new());

        self.change::<HudScreen>();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        // update pixel scale and screen size
        self.pixel_scale = 3.0;

        if width <= 1000.0 || height <= 750.0 { self.pixel_scale = 2.0 }
        if width >= 2200.0 || height >= 1200.0 { self.pixel_scale = 4.0 }
        if width >= 2800.0 || height >= 1800.0 { self.pixel_scale = 6.0 }

        self.screen_size = Vec2::new(width, height) / self.pixel_scale;


        self.ui_renderer.resize(width, height, self.pixel_scale);

        let info = self.get_current_screen_info();
        info.borrow_mut().screen_size = self.screen_size;
        info.borrow_mut().screen_center = self.screen_size / 2.0;
        self.get_current_screen().borrow_mut().resize(&info.borrow());
    }

    pub fn update(&mut self, dt: f32) {
        let info = self.get_current_screen_info();
        self.get_current_screen().borrow_mut().update(dt, &info.borrow());
    }

    pub fn draw(&mut self) {
        self.get_current_screen().borrow_mut().draw(&mut self.ui_renderer);

        self.ui_renderer.draw();
    }

    pub fn change<T>(&mut self)
    where
        T: ScreenBase,
        for<'a> T: 'a
    {
        let new_screen = self.screens[&TypeId::of::<T>()].clone();

        Self::change_logic(self.screen_size, &new_screen, &self.resource_manager.as_ref().unwrap().borrow());
        self.current_screen = Some(new_screen);
    }

    fn change_logic(screen_size: Vec2, screen_info: &MutRc<ScreenInfo>, resource_manager: &ResourceManager) {
        let info = &mut screen_info.borrow_mut();

        if !info.started {
            info.started = true;
            info.screen_size = screen_size;
            info.screen_center = screen_size / 2.0;
            info.screen.borrow_mut().start(resource_manager, &info);

            // not resize if screen size in zero
            if screen_size != Vec2::ZERO {
                info.screen.borrow_mut().resize(&info);
            }
        }

        if info.screen_size != screen_size {
            info.screen_size = screen_size;
            info.screen_center = screen_size / 2.0;
            info.screen.borrow_mut().resize(&info);
        }
    }

    pub fn add<T>(&mut self, screen: T)
    where
        T: ScreenBase,
        for<'a> T: 'a
    {
        let screen_info = ScreenInfo::new(Rc::new(RefCell::new(screen)));
        self.screens.insert(TypeId::of::<T>(), Rc::new(RefCell::new(screen_info)));
    }

    pub fn get_current_screen(&self) -> MutRc<dyn ScreenBase> {
        self.current_screen.as_ref().unwrap().borrow_mut().screen.clone()
    }

    pub fn get_current_screen_info(&self) -> MutRc<ScreenInfo> {
        self.current_screen.as_ref().unwrap().clone()
    }
}
