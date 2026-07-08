use std::array;

use crate::world::particles::{ParticlesManager, ParticlesSpawnArgs};
use crate::{inputs, math};
use crate::inputs::MouseButton;
use crate::math::Vec3;
use crate::world::blocks::BlocksManager;
use crate::world::{Aabb, Chunk, Planet};
use crate::world::chunk::ChunkGetter;
use crate::world::player::camera::Camera;
use crate::world::player::{EntityInventory, ItemStack, SelectionBox};
use crate::world::player::entitiy_inventory::{PLAYER_HOTBAR_SLOTS_COUNT, PLAYER_SLOTS_COUNT_TOTAL};


const GRAVITY: f32 = 35.0;
const JUMP_FORCE: f32 = 10.0;
const FLY_Y_SPEED: f32 = 120.0;
const FLY_X_SPEED: f32 = 320.0;
const SPEED: f32 = 50.0;

pub struct Player {
    pub camera: Camera,

    selected_hotbar_slot: i32,
    inventory: [ItemStack; PLAYER_SLOTS_COUNT_TOTAL],

    pub selection_box: SelectionBox,

    aabb: Aabb,
    velocity: Vec3,

    flying_mode: bool,
    on_ground: bool,
}

impl EntityInventory for Player {
    fn get_slot(&self, index: i32) -> &ItemStack {
        &self.inventory[index as usize]
    }

    fn get_selected_hotbar_index(&self) -> i32 {
        self.selected_hotbar_slot
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),

            selected_hotbar_slot: 0,
            inventory: array::from_fn(|_| ItemStack::EMPTY),

            selection_box: SelectionBox::new(),

            aabb: Aabb::new(0.0, 0.0, 0.0, 0.6, 1.8, 0.6).clone_move(0.0, 60.0, 0.0),
            velocity: Vec3::ZERO,

            flying_mode: false,
            on_ground: false,
        }
    }

    pub fn reset(&mut self) {
        self.aabb.set_position(0.0, 60.0, 0.0);
    }

    pub fn get_pos(&self) -> Vec3 {
        self.aabb.get_pos()
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.camera.start();

        self.inventory[0] = ItemStack::new(blocks_manager.grass_block.get_base(), 64);
        self.inventory[1] = ItemStack::new(blocks_manager.cobblestone.get_base(), 64);
        self.inventory[2] = ItemStack::new(blocks_manager.bedrock.get_base(), 64);
        self.inventory[3] = ItemStack::new(blocks_manager.stone.get_base(), 64);
        self.inventory[4] = ItemStack::new(blocks_manager.ice_block.get_base(), 64);
        self.inventory[5] = ItemStack::new(blocks_manager.red_flower.get_base(), 64);
        self.inventory[6] = ItemStack::new(blocks_manager.snow_layer.get_base(), 64);
        self.inventory[7] = ItemStack::new(blocks_manager.water_block.get_base(), 64);
        self.inventory[8] = ItemStack::new(blocks_manager.dirt.get_base(), 64);
    }

    pub fn update(&mut self, dt: f32, planet: &mut Planet, blocks_manager: &BlocksManager, particles_manager: &mut ParticlesManager) {
        self.process_input(dt);
        self.process_collision(dt, planet, blocks_manager);

        self.camera.update(Vec3::new(
            self.aabb.get_pos().x + self.aabb.get_size().x / 2.0,
            self.aabb.get_pos().y + 1.7,
            self.aabb.get_pos().z + self.aabb.get_size().z / 2.0,
        ));


        let ray_pos = self.update_ray_casting(planet, blocks_manager, self.camera.get_pos(), self.camera.get_dir(), particles_manager);
        self.selection_box.update(dt, ray_pos);


        // update hotbar slot
        self.selected_hotbar_slot -= inputs::get_mouse_scroll();

        if self.selected_hotbar_slot < 0 {
            self.selected_hotbar_slot = (PLAYER_HOTBAR_SLOTS_COUNT - 1) as i32;
        }
        else if self.selected_hotbar_slot >= PLAYER_HOTBAR_SLOTS_COUNT as i32 {
            self.selected_hotbar_slot = 0;
        }
    }

    fn update_ray_casting(&mut self, planet: &Planet, blocks_manager: &BlocksManager, start: Vec3, dir: Vec3, particles_manager: &mut ParticlesManager) -> Option<Vec3> {
        const RAY_LENGHT: f32 = 4.5;
        const RAY_STEP: f32 = 0.1;

        let mut target_pos: Option<Vec3> = None;
        let mut ch = ChunkGetter::new(None);

        let mut step = 0.0f32;
        while step < RAY_LENGHT {
            let pos = start + dir * step;

            let chunk_pos = math::get_chunk_pos(pos);
            ch.change(chunk_pos, planet);

            if let Some(ref chunk) = ch.chunk {
                let chunk_block = math::get_chunk_block(chunk_pos, pos);
                let block_info = chunk.borrow().chunk_data.get_block_info(chunk_block);

                let block_properties = blocks_manager.get_properties_from_block_info(block_info);

                if block_properties.selection_box.is_some() {
                    // break block
                    if inputs::mouse_button_pressed(MouseButton::Left) {
                        chunk.borrow_mut().chunk_data.set_block(chunk_block, blocks_manager.air.get_properties(0));
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

        let hand_slot_item = self.get_selected_hotbar_slot();

        if !inputs::mouse_button_pressed(MouseButton::Right) || hand_slot_item.is_empty() || !hand_slot_item.get_item().is_block() {
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

            let block_properties = blocks_manager.get_properties_from_block_info(block_info);

            if block_properties.can_replaced {
                let hand_slot_item = self.get_selected_hotbar_slot().get_item();

                let block_properties = blocks_manager.get_properties_from_item_base(hand_slot_item);
                chunk.borrow_mut().chunk_data.set_block(chunk_block, block_properties);
            }
        }

        return target_pos;
    }

    fn process_input(&mut self, dt: f32) {
        let mut dir = Vec3::ZERO;

        let yaw = self.camera.rot.x.to_radians();
        let front = Vec3 { x: yaw.cos(), y: 0.0, z: yaw.sin() };

        if inputs::key_down(inputs::Keys::W) { dir = dir + front };
        if inputs::key_down(inputs::Keys::A) { dir = dir - front.cross(Vec3::UP) };
        if inputs::key_down(inputs::Keys::S) { dir = dir - front };
        if inputs::key_down(inputs::Keys::D) { dir = dir + front.cross(Vec3::UP) };
        if inputs::key_down(inputs::Keys::Space) && self.on_ground { self.velocity.y = JUMP_FORCE };

        if inputs::key_pressed(inputs::Keys::F) { self.flying_mode = !self.flying_mode }

        if self.flying_mode {
            if inputs::key_down(inputs::Keys::LeftShift) { self.velocity.y -= FLY_Y_SPEED * dt };
            if inputs::key_down(inputs::Keys::Space) { self.velocity.y += FLY_Y_SPEED * dt };
        }


        if dir.length() > 1.0 { dir = dir.normalized() }

        let speed = if self.flying_mode { FLY_X_SPEED } else { SPEED };
        self.velocity += dir * (speed * dt);
    }

    fn process_collision(&mut self, dt: f32, planet: &mut Planet, blocks_manager: &BlocksManager) {
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

        let xaOrg = xa;
        let yaOrg = ya;
        let zaOrg = za;

        //let cubes = planet.get_cubes(blocks_manager, &self.aabb.expand(xa, ya, za));

        //for cube in cubes { ya = cube.clip_y_collide(&self.aabb, ya) }
        self.aabb.move_at(0.0, ya, 0.0);

        //for cube in cubes { xa = cube.clip_x_collide(&self.aabb, xa) }
        self.aabb.move_at(xa, 0.0, 0.0);

        //for cube in cubes { za = cube.clip_z_collide(&self.aabb, za) }
        self.aabb.move_at(0.0, 0.0, za);


        let og = self.on_ground || (yaOrg != ya && yaOrg < 0.0);

        let foot_size = 0.5f32;

        if foot_size > 0.0 && og && ((xaOrg != xa) || (zaOrg != za)) {
            let xaN = xa;
            let yaN = ya;
            let zaN = za;

            xa = xaOrg;
            ya = foot_size;
            za = zaOrg;

            let normal = self.aabb;
            self.aabb.set(&org);

            //let cubes = planet.get_cubes(blocks_manager, &self.aabb.expand(xa, ya, za));

            //for cube in cubes { ya = cube.clip_y_collide(&self.aabb, ya) }
            self.aabb.move_at(0.0, ya, 0.0);

            //for cube in cubes { xa = cube.clip_x_collide(&self.aabb, xa) }
            self.aabb.move_at(xa, 0.0, 0.0);

            //for cube in cubes { za = cube.clip_z_collide(&self.aabb, za) }
            self.aabb.move_at(0.0, 0.0, za);

            if xaN * xaN + zaN * zaN >= xa * xa + za * za {
                xa = xaN;
                ya = yaN;
                za = zaN;
                self.aabb.set(&normal);
            }
        }

        self.on_ground = yaOrg != ya && yaOrg < 0.0;

        if xaOrg != xa { self.velocity.x = 0.0 }
        if yaOrg != ya { self.velocity.y = 0.0 }
        if zaOrg != za { self.velocity.z = 0.0 }
    }
}
