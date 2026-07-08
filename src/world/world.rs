use crate::{inputs, render::{GlobalRenderer, Ubo}, resources::ResourceManager, world::{Planet, Player, blocks::BlocksManager, particles::ParticlesManager, sky::Sky}};


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
        self.blocks_manager = Some(BlocksManager::new(resources_manager));

        self.player.start(&self.blocks_manager.as_ref().unwrap());

        self.planet.start();

        self.sky.start(resources_manager, global_renderer);
        self.particles_manager.start(resources_manager, global_renderer);
        self.player.selection_box.start(global_renderer);
    }

    pub fn update(&mut self, dt: f32) {
        self.player.update(dt, &mut self.planet, &self.blocks_manager.as_ref().unwrap(), &mut self.particles_manager);

        self.sky.update(dt, self.player.get_pos(), self.planet.render_distance);

        self.planet.update(self.player.get_pos(), &self.blocks_manager.as_ref().unwrap());

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
        self.planet.draw(&self.player.camera, &self.blocks_manager.as_ref().unwrap(), global_renderer);
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
        self.planet.load_chunks(self.player.get_pos(), &self.blocks_manager.as_ref().unwrap());
    }
}
