use crate::game::{GameEvents, PlayerStates};
use crate::ui::ui_manager::ScreensId;
use crate::utils::SafePtr;
use crate::world::blocks::BlockProperties;
use crate::world::particles::{ParticlesManager, ParticlesSpawnArgs};
use crate::world::world::WorldUpdateArgs;
use crate::{inputs, math};
use crate::inputs::Inputs;
use crate::math::Vec3;
use crate::world::{Aabb, Planet};
use crate::world::player::camera::Camera;
use crate::world::player::{PlayerInventory, SelectionBox};


const GRAVITY: f32 = 35.0;
const JUMP_FORCE: f32 = 10.0;
const FLY_Y_SPEED: f32 = 120.0;
const FLY_X_SPEED: f32 = 320.0;
const SPEED: f32 = 50.0;
const SWIM_SPEED_UP: f32 = 10.0;


#[derive(Clone)]
pub struct RaycastingResult {
    pub block_pos: Vec3,
    pub hit_normal: Vec3,
    pub block_properties: SafePtr<BlockProperties>,
    pub block_selection_box: Aabb,
}

pub struct Player {
    pub camera: Camera,

    pub inventory: PlayerInventory,

    pub selection_box: SelectionBox,

    aabb: Aabb,
    velocity: Vec3,

    in_water: bool,
    flying_mode: bool,
    on_ground: bool,
    pub state: PlayerStates,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),

            inventory: PlayerInventory::new(),

            selection_box: SelectionBox::new(),

            aabb: Aabb::new(0.0, 0.0, 0.0, 0.6, 1.8, 0.6).clone_move(0.0, 60.0, 0.0),
            velocity: Vec3::ZERO,

            in_water: false,
            flying_mode: false,
            on_ground: false,
            state: PlayerStates::Menu,
        }
    }

    pub fn reset(&mut self) {
        self.aabb.set_position(0.0, 60.0, 0.0);
    }

    pub fn get_pos(&self) -> Vec3 {
        self.aabb.get_min()
    }

    pub fn start(&mut self) {
        self.camera.start();
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs, planet: &mut Planet, particles_manager: &mut ParticlesManager) {
        self.state = if args.current_screen_id == ScreensId::HudScreen {
             PlayerStates::Active
        } else { PlayerStates::Menu };


        self.process_input(args);
        self.process_collision(args.dt, planet, args.inputs);

        self.in_water = false;
        planet.iterate_over_blocks_cube(&self.aabb, |stop, _, blocks_manager, _, _, _, block_properties| {
            if *block_properties == blocks_manager.water_block {
                self.in_water = true;
                *stop = true;
            }
        });

        self.camera.update(&self.aabb, planet, args.inputs.get_camera_delta());

        //let now = std::time::Instant::now();
        let ray_result = self.update_ray_casting(planet, particles_manager, args.inputs);

        self.selection_box.update(&ray_result);

        if self.state == PlayerStates::Active {
            self.inventory.process_hotbar_scroll(args.inputs.get_mouse_scroll());

            if let Some(result) = ray_result {
                let slot = self.inventory.get_selected_hotbar_slot();

                if args.inputs.mouse_pressed(inputs::MouseButton::Right) && let Some(item) = slot.get_item() && item.is_block() {
                    let keep_same_block = result.block_properties.can_replace && (item.id != result.block_properties.base_properties.id);
                    let place_block = if keep_same_block { result.block_pos } else { result.block_pos + result.hit_normal };

                    let chunk_pos = math::get_chunk_pos(place_block);
                    let chunk_block = math::get_chunk_block(chunk_pos, place_block);

                    if let Some(chunk) = planet.get_chunk(chunk_pos) {
                        let block_properties = chunk.borrow().chunk_data.read().unwrap().get_block_properties(chunk_block);

                        if block_properties.can_replace {
                            let block_functions = planet.blocks_manager.get_from_item_base(item);
                            chunk.borrow_mut().chunk_data.write().unwrap().set_block(chunk_block, block_functions.get_id_state());
                        }
                    }
                }
            }
        }
        //println!("{}", now.elapsed().as_micros());


        if args.inputs.key_pressed(inputs::Keys::E) {
            args.events_queue.push_back(GameEvents::ChangeScreen(ScreensId::InventoryScreen));
        }
    }

    fn update_ray_casting(&mut self,
        planet: &Planet,
        particles_manager: &mut ParticlesManager,
        inputs: &Inputs
    ) -> Option<RaycastingResult> {
        let mut result = None;

        const RAY_LENGHT: f32 = 4.5;
        let ray_pos = self.camera.get_pos();
        let ray_dir = self.camera.get_dir();

        planet.iterate_over_blocks_raycast(ray_pos, ray_dir, RAY_LENGHT, |stop, it| {
            if let Some(ref selection_box) = it.block_properties.selection_box {
                let target_block = it.global_block.as_vec3();

                let aabb = selection_box.clone_movev(target_block);

                if let Some(hit) = aabb.ray_intersect(ray_pos, ray_dir) {
                    // break block
                    if inputs.mouse_pressed(inputs::MouseButton::Left) && self.state == PlayerStates::Active {
                        let block_properties = it.chunk.borrow().chunk_data.read().unwrap().get_block_properties(it.chunk_block);
                        particles_manager.spawn(ParticlesSpawnArgs::BlockDestroy(&block_properties, target_block));

                        it.chunk.borrow().chunk_data.write().unwrap().set_block(it.chunk_block, it.blocks_manager.air);
                    }

                    result = Some(RaycastingResult{
                        block_pos: target_block,
                        hit_normal: aabb.get_ray_hit_normal(hit),
                        block_properties: it.block_properties.clone(),
                        block_selection_box: aabb,
                    });

                    *stop = true;
                }
            }
        });

        return result;
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
        if args.inputs.key_down(inputs::Keys::Space) {
            if self.in_water && self.velocity.y < 3.0 {
                self.velocity.y += SWIM_SPEED_UP * args.dt;
            }
            else if self.on_ground {
                self.velocity.y = JUMP_FORCE;
            }
        }


        if args.inputs.key_pressed(inputs::Keys::F) { self.flying_mode = !self.flying_mode }

        if self.flying_mode {
            if args.inputs.key_down(inputs::Keys::LeftShift) { self.velocity.y -= FLY_Y_SPEED * args.dt };
            if args.inputs.key_down(inputs::Keys::Space) { self.velocity.y += FLY_Y_SPEED * args.dt };
        }


        if dir.length() > 1.0 { dir = dir.normalized() }

        let speed = if self.flying_mode { FLY_X_SPEED } else { SPEED };
        self.velocity += dir * (speed * args.dt);
    }

    fn process_collision(&mut self, dt: f32, planet: &mut Planet, inputs: &Inputs) {
        let friction = if self.in_water { math::FRICTION * 1.75 } else { math::FRICTION };

        self.velocity.x -= self.velocity.x * (friction * dt);
        self.velocity.z -= self.velocity.z * (friction * dt);
        if self.flying_mode {
            self.velocity.y -= self.velocity.y * (math::FRICTION * dt);
        }
        else {
            if self.in_water && !inputs.key_down(inputs::Keys::Space) && self.velocity.y > -4.0 {
                self.velocity.y -= GRAVITY * 0.1 * dt;
            }
            else if !self.in_water{
                self.velocity.y -= GRAVITY * dt;
            }
        }


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

        let cubes = planet.get_blocks_hitboxes(&self.aabb.expand(xa, ya, za));

        for cube in cubes { ya = cube.clip_y_collide(&self.aabb, ya) }
        self.aabb.move_at(0.0, ya, 0.0);

        for cube in cubes { xa = cube.clip_x_collide(&self.aabb, xa) }
        self.aabb.move_at(xa, 0.0, 0.0);

        for cube in cubes { za = cube.clip_z_collide(&self.aabb, za) }
        self.aabb.move_at(0.0, 0.0, za);


        let og = self.on_ground || (ya_org != ya && ya_org < 0.0);

        let foot_size = 0.5;

        if foot_size > 0.0 && og && ((xa_org != xa) || (za_org != za)) {
            let xa_n = xa;
            let ya_n = ya;
            let za_n = za;

            xa = xa_org;
            ya = foot_size;
            za = za_org;

            let normal = self.aabb;
            self.aabb.set(&org);

            let cubes = planet.get_blocks_hitboxes(&self.aabb.expand(xa, ya, za));

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
