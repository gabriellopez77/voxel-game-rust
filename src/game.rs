use std::{rc::Rc, cell::RefCell};
use crate::render::ChunkVertices;
use crate::utils::ObjectPool;
use crate::world::sky::Sky;
use crate::{inputs, window};
use crate::resources::ResourceManager;
use crate::ui::screens_manager::ScreenManager;
use crate::world::{Player, Planet, blocks::BlocksManager};


pub struct Game {
    player: Player,
    planet: Planet,
    sky: Sky,

    screen_manager: ScreenManager,
    resource_manager: Rc<RefCell<ResourceManager>>,
    blocks_manager: BlocksManager,

    paused: bool,

    chunk_mesh_vertices_pool: ObjectPool<Vec<ChunkVertices>>,
    chunk_mesh_indices_pool: ObjectPool<Vec<u32>>,
}


impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            planet: Planet::new(),
            sky: Sky::new(),

            screen_manager: ScreenManager::new(),
            resource_manager: Rc::new(RefCell::new(ResourceManager::new())),
            blocks_manager: BlocksManager::new(),

            paused: false,

            chunk_mesh_vertices_pool: ObjectPool::new(),
            chunk_mesh_indices_pool: ObjectPool::new(),
        }
    }

    pub fn start(&mut self) {
        self.resource_manager.borrow_mut().start();
        self.blocks_manager.start(&self.resource_manager);
        self.screen_manager.start(self.resource_manager.clone());

        self.player.start(self.resource_manager.clone());
        self.planet.start(self.resource_manager.clone());

        self.sky.start(self.resource_manager.clone());


        //let now = std::time::Instant::now();

        //println!("{}", now.elapsed().as_micros());
    }

    pub fn update(&mut self, dt: f32, window: &mut window::Window) {
        if inputs::is_key_pressed(inputs::Keys::Escape) {
            self.paused = !self.paused;

            if self.paused {window.set_cursor(glfw::CursorMode::Normal);}
            else {window.set_cursor(glfw::CursorMode::Hidden);}
        }

        if !self.paused {
            //println!("vertices pool count: {}", self.chunk_mesh_vertices_pool.count());
            //println!("indices pool count: {}", self.chunk_mesh_indices_pool.count());
            //println!("chunks pool count: {}", self.planet.chunk_pool.count());

            self.player.update(dt);
            self.sky.update();
            self.planet.update(self.player.camera.position, &self.blocks_manager);
        }

        self.screen_manager.update(dt);
    }

    pub fn render(&mut self) {
        self.sky.draw();
        self.planet.draw(&self.player.camera, &self.blocks_manager, &mut self.chunk_mesh_vertices_pool, &mut self.chunk_mesh_indices_pool);

        self.player.camera.view_changed = false;
        self.screen_manager.draw();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.screen_manager.resize(width as f32, height as f32);
        self.player.camera.resize(width as f32, height as f32)
    }
}
