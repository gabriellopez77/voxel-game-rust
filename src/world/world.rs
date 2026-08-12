use std::collections::VecDeque;

use crate::{game::{GameEvents, PlayerStates}, inputs::Inputs, render::GlobalRenderer, resources::ResourceManager, ui::ui_manager::ScreensId, world::{Planet, Player, blocks::BlocksManager, particles::ParticlesManager, sky::Sky}};


pub struct WorldUpdateArgs<'a> {
    pub is_paused: bool,
    pub events_queue: &'a mut VecDeque<GameEvents>,
    pub inputs: &'a mut Inputs,
    pub dt: f32,
    pub current_screen_id: ScreensId,
    pub resources: &'a mut ResourceManager,
}

pub struct World {
    pub player: Player,

    pub planet: Planet,
    pub sky: Sky,
    pub particles_manager: ParticlesManager,

    pub blocks_manager: BlocksManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player::new(),

            planet: Planet::new(),
            sky: Sky::new(),
            particles_manager: ParticlesManager::new(),

            blocks_manager: BlocksManager::default(),
        }
    }

    pub fn start(&mut self, resources: &mut ResourceManager, global_renderer: &mut GlobalRenderer) {
        self.blocks_manager = BlocksManager::new(resources, &mut self.player.inventory);

        self.player.start();

        self.planet.start(&self.blocks_manager, global_renderer);

        self.sky.start(resources, global_renderer);
        self.particles_manager.start(resources, global_renderer);
        self.player.selection_box.start(global_renderer);
        self.player.first_person.start(global_renderer, resources);
    }

    pub fn update(&mut self, dt: f32, args: &mut WorldUpdateArgs) {
        args.inputs.reset_camera_delta(args.is_paused || self.player.state == PlayerStates::Menu);

        if args.is_paused {
            return;
        }

        self.player.update(args, &mut self.planet, &mut self.particles_manager);

        self.sky.update(dt, &self.player.camera, self.planet.render_distance);

        self.planet.update(self.player.get_pos());

        self.particles_manager.update(dt);
    }

    pub fn draw(&mut self, dt: f32, global_renderer: &mut GlobalRenderer) {
        self.sky.draw(global_renderer);
        self.player.selection_box.draw(global_renderer);
        self.player.first_person.draw(global_renderer);
        self.planet.draw(dt, &self.player.camera, global_renderer);
        self.particles_manager.draw(global_renderer, self.player.camera.rot);

        self.player.camera.view_changed = false;

        let ubo = &mut global_renderer.global_ubo;

        // cam matrix
        ubo.data.cam_view.0 = self.player.camera.view_matrix;
        ubo.data.cam_viewproj.0 = self.player.camera.viewproj_matrix;
        ubo.data.cam_view_no_translate.0 = self.player.camera.view_no_translate_matrix;
        ubo.data.cam_proj.0 = self.player.camera.projection_matrix;
        ubo.data.first_person_proj.0 = self.player.camera.first_person_projection_matrix;

        // sky
        ubo.data.fog_distance = self.sky.fog_norm_distance;
        ubo.data.fog_density = self.sky.fog_density;
        ubo.data.fog_enable = self.sky.fog_enabled as i32;
        ubo.data.sky_color.0 = self.sky.sky_color.normalized();
        ubo.data.fog_color.0 = self.sky.fog_color.normalized();
        ubo.data.clouds_color.0 = self.sky.clouds_color.normalized();

        // world
        ubo.data.render_distance = self.planet.render_distance as f32;
    }

    pub fn cleanup(&mut self) {
        self.planet.cleanup();
        self.planet.stop();
        self.sky.cleanup();
        self.player.selection_box.cleanup();
        self.player.first_person.cleanup();
        self.particles_manager.cleanup();
        self.planet.chunks_renderer.cleanup();
    }

    pub fn leave(&mut self) {
        self.player.reset();
        self.planet.cleanup();
        self.particles_manager.reset();
    }

    pub fn load(&mut self) {
        self.planet.load_chunks(self.player.get_pos());
    }
}
