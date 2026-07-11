use std::collections::VecDeque;

use crate::{game::GameEvents, inputs::Inputs, render::{GlobalRenderer, Ubo}, resources::ResourceManager, ui::ui_manager::ScreensId, world::{Planet, Player, blocks::BlocksManager, particles::ParticlesManager, sky::Sky}};

pub struct WorldUpdateArgs<'a> {
    pub events_queue: &'a mut VecDeque<GameEvents>,
    pub inputs: &'a Inputs,
    pub dt: f32,
    pub current_screen_id: ScreensId,
}

pub struct World {
    pub player: Player,

    pub planet: Planet,
    pub sky: Sky,
    pub particles_manager: ParticlesManager,

    pub blocks_manager: Option<BlocksManager>,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player::new(),

            planet: Planet::new(),
            sky: Sky::new(),
            particles_manager: ParticlesManager::new(),

            blocks_manager: None,
        }
    }

    pub fn start(&mut self, resources_manager: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        self.blocks_manager = Some(BlocksManager::new(resources_manager, &mut self.player.inventory));

        self.player.start();

        self.planet.start(&self.blocks_manager.as_ref().unwrap());

        self.sky.start(resources_manager, global_renderer);
        self.particles_manager.start(resources_manager, global_renderer);
        self.player.selection_box.start(global_renderer);
    }

    pub fn update(&mut self, dt: f32, args: &mut WorldUpdateArgs) {
        self.player.update(args, &mut self.planet, &mut self.particles_manager);

        self.sky.update(dt, &self.player.camera, self.planet.render_distance);

        self.planet.update(self.player.get_pos());

        self.particles_manager.update(dt);
    }

    pub fn draw(&mut self, ubo: &mut Ubo, global_renderer: &mut GlobalRenderer) {
        // cam matrix
        ubo.update("camView", self.player.camera.view_matrix.as_ptr());
        ubo.update("camViewProj", self.player.camera.projection_view_matrix.as_ptr());
        ubo.update("camViewNoTranslate", self.player.camera.view_no_translate_matrix.as_ptr());
        ubo.update("camProj", self.player.camera.projection_matrix.as_ptr());

        // sky
        ubo.update("fogDistance", &self.sky.fog_norm_distance);
        ubo.update("fogDensity", &self.sky.fog_density);
        ubo.update("fogEnable", &self.sky.fog_enabled);
        ubo.update("skyColor", &self.sky.sky_color.normalized());
        ubo.update("fogColor", &self.sky.fog_color.normalized());
        ubo.update("cloudsColor", &self.sky.clouds_color.normalized());

        // world
        ubo.update("renderDistance", &(self.planet.render_distance as f32));


        self.sky.draw(global_renderer);
        self.player.selection_box.draw(global_renderer);
        self.planet.draw(&self.player.camera, global_renderer);
        self.particles_manager.draw(global_renderer, self.player.camera.rot);

        self.player.camera.view_changed = false;
    }

    pub fn cleanup(&mut self) {
        self.planet.cleanup();
        self.planet.stop();
        self.sky.cleanup();
        self.player.selection_box.cleanup();
        self.particles_manager.cleanup();
    }

    pub fn leave(&mut self) {
        self.player.reset();
        self.planet.cleanup();
    }

    pub fn load(&mut self) {
        self.planet.load_chunks(self.player.get_pos());
    }
}
