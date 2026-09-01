use std::collections::VecDeque;

use crate::{game::{GameEvents, PlayerStates}, inputs::Inputs, render::{ChunksRenderer, EntitiesRenderer, GlobalRenderer}, resources::ResourceManager, ui::ui_manager::ScreensId, world::{Planet, Player, blocks::BlocksManager, particles::ParticlesManager, sky::Sky}};
use crate::game::{GameFlags, GameStates};

pub struct WorldUpdateArgs<'a> {
    pub events_queue: &'a mut VecDeque<GameEvents>,
    pub inputs: &'a mut Inputs,
    pub dt: f32,
    pub time: f32,
    pub current_screen_id: ScreensId,
    pub resources: &'a mut ResourceManager,
    pub game_state: GameStates,
    pub game_flags: GameFlags,
}

pub struct World {
    pub player: Player,

    pub planet: Planet,
    pub sky: Sky,
    pub particles_manager: ParticlesManager,

    pub blocks_manager: BlocksManager,

    pub chunks_renderer: ChunksRenderer,
    pub entities_renderer: EntitiesRenderer,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player::new(),

            planet: Planet::new(),
            sky: Sky::new(),
            particles_manager: ParticlesManager::new(),

            blocks_manager: BlocksManager::default(),

            chunks_renderer: ChunksRenderer::new(),
            entities_renderer: EntitiesRenderer::new(),
        }
    }

    pub fn start(&mut self, resources: &mut ResourceManager, global_renderer: &mut GlobalRenderer) {
        self.blocks_manager = BlocksManager::new(resources, &mut self.player.inventory);

        self.player.start(resources, global_renderer);
        self.planet.start(&self.blocks_manager);

        self.sky.start(resources, global_renderer);
        self.particles_manager.start(resources, global_renderer);


        self.chunks_renderer.start(&self.blocks_manager, global_renderer);
        self.entities_renderer.start(global_renderer);
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs) {
        if args.game_state == GameStates::Loading {
            self.planet.chunks_manager.process_load_chunks();

            if self.planet.chunks_manager.get_pendings_chunks_count() == 0 {
                args.events_queue.push_back(GameEvents::EnterToWorld);
            }

            return;
        }

        if args.game_flags.contains(GameFlags::PAUSED) || self.player.state == PlayerStates::Menu {
            args.inputs.reset_camera_delta();
        }

        if args.game_flags.contains(GameFlags::PAUSED) {
            return;
        }

        self.player.update(args, &mut self.planet, &mut self.particles_manager);

        self.sky.update(args.dt, &self.player.camera, self.planet.render_distance);

        self.planet.update(self.player.get_pos());

        self.particles_manager.update(args.dt);
    }

    pub fn draw(&mut self, dt: f32, global_renderer: &mut GlobalRenderer) {
        self.chunks_renderer.process_mesh_worker();

        self.sky.draw(global_renderer);

        self.player.draw(&mut self.entities_renderer, global_renderer);

        self.planet.draw(dt, &self.player.camera, &mut self.chunks_renderer);
        self.particles_manager.draw(global_renderer, self.player.camera.get_rot());

        self.player.camera.view_changed = false;

        self.chunks_renderer.draw(global_renderer);
        self.entities_renderer.draw(global_renderer);

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
        ubo.data.light_color.0 = self.sky.light_color.normalized();
        ubo.data.darkness_color.0 = self.sky.darkness_color.normalized();
        ubo.data.ambient_color.0 = self.sky.ambient_color.normalized();

        // world
        ubo.data.render_distance = self.planet.render_distance as f32;
    }

    pub fn cleanup(&mut self) {
        self.planet.cleanup(&mut self.chunks_renderer);
        self.planet.stop();
        self.player.cleanup();
        self.sky.cleanup();
        self.particles_manager.cleanup();

        self.chunks_renderer.stop_mesh_worker();
        self.chunks_renderer.cleanup();
        self.entities_renderer.cleanup();
    }

    pub fn leave(&mut self) {
        self.player.reset();

        self.planet.cleanup(&mut self.chunks_renderer);
        self.chunks_renderer.clean_worker();

        self.particles_manager.reset();
    }

    pub fn load(&mut self) {
        self.planet.load_chunks(self.player.get_pos());
    }
}
