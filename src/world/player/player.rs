use crate::game::GameEvents;
use crate::ui::ui_manager::ScreensId;
use crate::world::particles::{ParticlesManager, ParticlesSpawnArgs};
use crate::world::world::WorldUpdateArgs;
use crate::{inputs, math};
use crate::inputs::{Inputs, MouseButton};
use crate::math::Vec3;
use crate::world::{Aabb, Chunk, Planet};
use crate::world::chunk::ChunkGetter;
use crate::world::player::camera::Camera;
use crate::world::player::{PlayerInventory, SelectionBox};


const GRAVITY: f32 = 35.0;
const JUMP_FORCE: f32 = 10.0;
const FLY_Y_SPEED: f32 = 120.0;
const FLY_X_SPEED: f32 = 320.0;
const SPEED: f32 = 50.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerStates {
    Active,
    Menu,
}

pub struct Player {
    pub camera: Camera,

    pub inventory: PlayerInventory,

    pub selection_box: SelectionBox,

    aabb: Aabb,
    velocity: Vec3,

    flying_mode: bool,
    on_ground: bool,
    state: PlayerStates,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),

            inventory: PlayerInventory::new(),

            selection_box: SelectionBox::new(),

            aabb: Aabb::new(0.0, 0.0, 0.0, 0.6, 1.8, 0.6).clone_move(0.0, 60.0, 0.0),
            velocity: Vec3::ZERO,

            flying_mode: false,
            on_ground: false,
            state: PlayerStates::Menu,
        }
    }

    pub fn reset(&mut self) {
        self.aabb.set_position(0.0, 60.0, 0.0);
    }

    pub fn get_pos(&self) -> Vec3 {
        self.aabb.get_pos()
    }

    pub fn start(&mut self) {
        self.camera.start();
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs, planet: &mut Planet, particles_manager: &mut ParticlesManager) {
        self.state = if args.current_screen_id == ScreensId::HudScreen {
             PlayerStates::Active
        } else { PlayerStates::Menu };

        self.process_input(args);
        self.process_collision(args.dt, planet);

        if self.state == PlayerStates::Active {
            self.inventory.process_hotbar_scroll(args.inputs.get_mouse_scroll());
        }

        self.camera.update(&self.aabb, planet, args.inputs.get_mouse_pos(), self.state);


        let ray_pos = self.update_ray_casting(planet, particles_manager, args.inputs);
        self.selection_box.update(args.dt, ray_pos);

        if args.inputs.key_pressed(inputs::Keys::E) {
            args.events_queue.push_back(GameEvents::ChangeScreen(ScreensId::InventoryScreen));
        }
    }

    fn update_ray_casting(&mut self, planet: &Planet, particles_manager: &mut ParticlesManager, inputs: &Inputs) -> Option<Vec3> {
        const RAY_LENGHT: f32 = 4.5;
        const RAY_STEP: f32 = 0.1;

        let start = self.camera.get_pos();
        let dir = self.camera.get_dir();

        let mut target_pos: Option<Vec3> = None;
        let mut ch = ChunkGetter::new();

        let mut step = 0.0f32;
        while step < RAY_LENGHT {
            let pos = start + dir * step;

            let chunk_pos = math::get_chunk_pos(pos);
            ch.change(chunk_pos, planet);

            if let Some(ref chunk) = ch.chunk {
                let chunk_block = math::get_chunk_block(chunk_pos, pos);
                let block_info = chunk.borrow().chunk_data.get_block_info(chunk_block);

                let block_properties = planet.blocks_manager.get_properties_from_block_info(block_info);

                if block_properties.selection_box.is_some() {
                    // break block
                    if inputs.mouse_pressed(MouseButton::Left) && self.state == PlayerStates::Active {
                        chunk.borrow_mut().chunk_data.set_block(chunk_block, planet.blocks_manager.air.get_properties(0));
                        particles_manager.spawn(ParticlesSpawnArgs::BlockDestroy(block_properties, (chunk_pos * Chunk::CHUNK_SIZE + chunk_block).as_vec3()));
                        break;
                    }

                    target_pos = Some((chunk_pos * Chunk::CHUNK_SIZE + chunk_block).as_vec3());
                    break;
                }
            }

            step += RAY_STEP;
        }

        if target_pos.is_none() { return None }

        let hand_slot_item = self.inventory.get_selected_hotbar_slot();

        if !inputs.mouse_pressed(MouseButton::Right) ||
            hand_slot_item.is_empty() ||
           !hand_slot_item.get_item().is_block() ||
            self.state == PlayerStates::Menu {
            return target_pos;
        }


        // place block
        let end = start + dir * step;
        let pos = end - dir * RAY_STEP;

        let chunk_pos = math::get_chunk_pos(pos);
        ch.change(chunk_pos, planet);

        if let Some(ref chunk) = ch.chunk {
            let chunk_block = math::get_chunk_block(chunk_pos, pos);
            let block_info = chunk.borrow().chunk_data.get_block_info(chunk_block);

            let block_properties = planet.blocks_manager.get_properties_from_block_info(block_info);

            if block_properties.can_replaced {
                let hand_slot_item = self.inventory.get_selected_hotbar_slot().get_item();

                let block_properties = planet.blocks_manager.get_properties_from_item_base(hand_slot_item);
                chunk.borrow_mut().chunk_data.set_block(chunk_block, block_properties);
            }
        }

        return target_pos;
    }

    fn process_input(&mut self, args: &mut WorldUpdateArgs) {
        if self.state == PlayerStates::Menu { return }

        let mut dir = Vec3::ZERO;

        let yaw = self.camera.rot.x.to_radians();
        let front = Vec3 { x: yaw.cos(), y: 0.0, z: yaw.sin() };

        if args.inputs.key_down(inputs::Keys::W) { dir = dir + front };
        if args.inputs.key_down(inputs::Keys::A) { dir = dir - front.cross(Vec3::UP) };
        if args.inputs.key_down(inputs::Keys::S) { dir = dir - front };
        if args.inputs.key_down(inputs::Keys::D) { dir = dir + front.cross(Vec3::UP) };
        if args.inputs.key_down(inputs::Keys::Space) && self.on_ground { self.velocity.y = JUMP_FORCE };

        if args.inputs.key_pressed(inputs::Keys::F) { self.flying_mode = !self.flying_mode }

        if self.flying_mode {
            if args.inputs.key_down(inputs::Keys::LeftShift) { self.velocity.y -= FLY_Y_SPEED * args.dt };
            if args.inputs.key_down(inputs::Keys::Space) { self.velocity.y += FLY_Y_SPEED * args.dt };
        }


        if dir.length() > 1.0 { dir = dir.normalized() }

        let speed = if self.flying_mode { FLY_X_SPEED } else { SPEED };
        self.velocity += dir * (speed * args.dt);
    }

    fn process_collision(&mut self, dt: f32, planet: &mut Planet) {
        self.velocity.x -= self.velocity.x * (math::FRICTION * dt);
        self.velocity.y -= if self.flying_mode { self.velocity.y * (math::FRICTION * dt) } else { GRAVITY * dt };
        self.velocity.z -= self.velocity.z * (math::FRICTION * dt);


        // epsilon
        if self.velocity.x.abs() < math::EPSILON { self.velocity.x = 0.0 }
        if self.velocity.y.abs() < math::EPSILON { self.velocity.y = 0.0 }
        if self.velocity.z.abs() < math::EPSILON { self.velocity.z = 0.0 }

        let org = self.aabb;

        let mut xa = self.velocity.x * dt;
        let mut ya = self.velocity.y * dt;
        let mut za = self.velocity.z * dt;

        let xa_org = xa;
        let ya_org = ya;
        let za_org = za;

        let cubes = planet.get_cubes(&self.aabb.expand(xa, ya, za));

        for cube in cubes { ya = cube.clip_y_collide(&self.aabb, ya) }
        self.aabb.move_at(0.0, ya, 0.0);

        for cube in cubes { xa = cube.clip_x_collide(&self.aabb, xa) }
        self.aabb.move_at(xa, 0.0, 0.0);

        for cube in cubes { za = cube.clip_z_collide(&self.aabb, za) }
        self.aabb.move_at(0.0, 0.0, za);


        let og = self.on_ground || (ya_org != ya && ya_org < 0.0);

        let foot_size = 0.5f32;

        if foot_size > 0.0 && og && ((xa_org != xa) || (za_org != za)) {
            let xa_n = xa;
            let ya_n = ya;
            let za_n = za;

            xa = xa_org;
            ya = foot_size;
            za = za_org;

            let normal = self.aabb;
            self.aabb.set(&org);

            let cubes = planet.get_cubes(&self.aabb.expand(xa, ya, za));

            for cube in cubes { ya = cube.clip_y_collide(&self.aabb, ya) }
            self.aabb.move_at(0.0, ya, 0.0);

            for cube in cubes { xa = cube.clip_x_collide(&self.aabb, xa) }
            self.aabb.move_at(xa, 0.0, 0.0);

            for cube in cubes { za = cube.clip_z_collide(&self.aabb, za) }
            self.aabb.move_at(0.0, 0.0, za);

            if xa_n * xa_n + za_n * za_n >= xa * xa + za * za {
                xa = xa_n;
                ya = ya_n;
                za = za_n;
                self.aabb.set(&normal);
            }
        }

        self.on_ground = ya_org != ya && ya_org < 0.0;

        if xa_org != xa { self.velocity.x = 0.0 }
        if ya_org != ya { self.velocity.y = 0.0 }
        if za_org != za { self.velocity.z = 0.0 }
    }
}
