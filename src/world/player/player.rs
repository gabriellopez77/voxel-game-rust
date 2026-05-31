use std::array;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{inputs, math};
use crate::inputs::MouseButton;
use crate::math::Vec3;
use crate::resources::ResourceManager;
use crate::world::blocks::BlocksManager;
use crate::world::{Chunk, Planet};
use crate::world::player::camera::Camera;
use crate::world::player::{EntityInventory, ItemStack, SelectionBox};
use crate::world::player::entitiy_inventory::{PLAYER_HOTBAR_SLOTS_COUNT, PLAYER_SLOTS_COUNT_TOTAL};

pub struct Player {
    pub camera: Camera,

    selected_hotbar_slot: i32,
    inventory: [ItemStack; PLAYER_SLOTS_COUNT_TOTAL],

    pub selection_box: SelectionBox,
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
            inventory: array::from_fn(|_| ItemStack::new()),

            selection_box: SelectionBox::new(),
        }
    }

    pub fn start(&mut self, resources: &ResourceManager) {
        self.camera.start(resources);
        self.camera.position.y = 60.0;

        self.selection_box.start(resources);
    }

    pub fn update(&mut self, dt: f32, planet: &Planet, blocks_manager: &BlocksManager) {
        let mut dir = Vec3::ZERO;

        let yaw = self.camera.rot.x.to_radians();
        let front = Vec3 { x: yaw.cos(), y: 0.0, z: yaw.sin() };

        if inputs::key_down(inputs::Keys::W) { dir = dir + front };
        if inputs::key_down(inputs::Keys::A) { dir = dir - front.cross(Vec3 { x: 0.0, y: 1.0, z: 0.0 }) };
        if inputs::key_down(inputs::Keys::S) { dir = dir - front };
        if inputs::key_down(inputs::Keys::D) { dir = dir + front.cross(Vec3 { x: 0.0, y: 1.0, z: 0.0 }) };
        if inputs::key_down(inputs::Keys::LeftShift) { dir.y -= 1.0 };
        if inputs::key_down(inputs::Keys::Space) { dir.y += 1.0 };

        const SPEED: f32 = 10.0;

        if dir.length() > 1.0 {
            dir = dir.normalized()
        }

        let new_pos = self.camera.position + dir * (SPEED * dt);
        self.camera.update(new_pos);

        let ray_pos = self.update_ray_casting(planet, blocks_manager, self.camera.position, self.camera.direction);
        self.selection_box.update(dt, ray_pos);

        // update hotbar slot
        self.selected_hotbar_slot -= inputs::get_mouse_scroll();
        
        if self.selected_hotbar_slot < 0 {
            self.selected_hotbar_slot = (PLAYER_HOTBAR_SLOTS_COUNT - 1) as i32;
        }
        if self.selected_hotbar_slot >= PLAYER_HOTBAR_SLOTS_COUNT as i32 {
            self.selected_hotbar_slot = 0;
        }
    }

    pub fn update_ray_casting(&mut self, planet: &Planet, blocks_manager: &BlocksManager, start: Vec3, dir: Vec3) -> Option<Vec3> {
        const RAY_LENGHT: f32 = 4.5;
        const RAY_STEP: f32 = 0.1;

        let mut step = 0.0f32;
        while step < RAY_LENGHT {
            let pos = start + dir * step;

            let chunk_pos = math::get_chunk_pos(pos);
            let ch = planet.get_chunk(chunk_pos);

            if let Some(chunk) = ch {
                let chunk_block = math::get_chunk_block(chunk_pos, pos);

                let block_wrapper = blocks_manager.get(chunk.borrow().chunk_data.get_block(chunk_block));
                let block_properties = block_wrapper.get_properties();

                if block_properties.base_properties.id != 0 {
                    // break block
                    if inputs::mouse_button_pressed(MouseButton::Left) {
                        chunk.borrow_mut().chunk_data.set_block(chunk_block, &blocks_manager.air);
                        return None;
                    }
                    else if inputs::mouse_button_pressed(MouseButton::Right) { // place block
                        
                    }
                    
                    return Some((chunk_pos * Chunk::CHUNK_SIZE + chunk_block).as_vec3());
                }
            }

            step += RAY_STEP;
        }

        return None;
    }
}
