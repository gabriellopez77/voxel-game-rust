use crate::game::{GameEvents, PlayerStates};
use crate::render::{EntitiesCubesVertices, EntitiesRenderer, GlobalRenderer};
use crate::resources::ResourceManager;
use crate::ui::ui_manager::ScreensId;
use crate::utils::SafePtr;
use crate::world::blocks::BlockProperties;
use crate::world::particles::ParticlesManager;
use crate::world::world::WorldUpdateArgs;
use crate::{inputs, math};
use crate::inputs::Inputs;
use crate::math::{Matrix4, Vec3};
use crate::world::{Aabb, Planet};
use crate::world::player::camera::{Camera, PerspectiveMode};
use crate::world::player::{FirstPerson, PlayerInventory, SelectionBox};


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

    selection_box: SelectionBox,
    first_person: FirstPerson,

    aabb: Aabb,
    velocity: Vec3,

    in_water: bool,
    flying_mode: bool,
    on_ground: bool,
    pub state: PlayerStates,

    head: EntitiesCubesVertices,
    left_leg: EntitiesCubesVertices,
    right_leg: EntitiesCubesVertices,
    body: EntitiesCubesVertices,
    left_arm: EntitiesCubesVertices,
    right_arm: EntitiesCubesVertices,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),

            inventory: PlayerInventory::new(),

            selection_box: SelectionBox::new(),
            first_person: FirstPerson::new(),

            aabb: Aabb::new(0.0, 0.0, 0.0, 0.6, 1.8, 0.6).clone_move(0.0, 60.0, 0.0),
            velocity: Vec3::ZERO,

            in_water: false,
            flying_mode: false,
            on_ground: false,
            state: PlayerStates::Menu,

            left_leg: EntitiesCubesVertices::default(),
            right_leg: EntitiesCubesVertices::default(),
            body: EntitiesCubesVertices::default(),
            left_arm: EntitiesCubesVertices::default(),
            right_arm: EntitiesCubesVertices::default(),
            head: EntitiesCubesVertices::default(),
        }
    }

    pub fn get_pos(&self) -> Vec3 { self.aabb.get_min() }

    pub fn start(&mut self, resources: &mut ResourceManager, global_renderer: &mut GlobalRenderer) {
        self.selection_box.start(global_renderer);
        self.first_person.start(global_renderer, resources);

        self.camera.start();

        //self.head.up_tex_coords = resources.world_texture.get_coords("grass_block_top");
        //self.head.down_tex_coords = resources.world_texture.get_coords("cobblestone");
        //self.head.south_tex_coords = resources.world_texture.get_coords("oak_log_side");
        self.head.north_tex_coords = resources.world_texture.get_coords("ice_block");
        //self.head.west_tex_coords = resources.world_texture.get_coords("dirt");
        //self.head.east_tex_coords = resources.world_texture.get_coords("dirt");
        self.head.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;

        {
            let mut matrix = Matrix4::IDENTITY;
            matrix.translatev(Vec3::new(-3.9, 0.0, -2.0) / 16.0);
            matrix.scalev(Vec3::new(4.0, 12.0, 4.0) / 16.0);

            self.left_leg.local_matrix = matrix;
            self.left_leg.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;
        }

        {
            let mut matrix = Matrix4::IDENTITY;
            matrix.translatev(Vec3::new(0.1, 0.0, -2.0) / 16.0);
            matrix.scalev(Vec3::new(4.0, 12.0, 4.0) / 16.0);

            self.right_leg.local_matrix = matrix;
            self.right_leg.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;
        }

        {
            let mut matrix = Matrix4::IDENTITY;
            matrix.translatev(Vec3::new(-4.0, 12.0, -2.0) / 16.0);
            matrix.scalev(Vec3::new(8.0, 12.0, 4.0) / 16.0);

            self.body.local_matrix = matrix;
            self.body.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;
        }

        {
            let mut matrix = Matrix4::IDENTITY;
            matrix.translatev(Vec3::new(-8.0, 12.0, -2.0) / 16.0);
            matrix.scalev(Vec3::new(4.0, 12.0, 4.0) / 16.0);

            self.left_arm.local_matrix = matrix;
            self.left_arm.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;
        }

        {
            let mut matrix = Matrix4::IDENTITY;
            matrix.translatev(Vec3::new(4.0, 12.0, -2.0) / 16.0);
            matrix.scalev(Vec3::new(4.0, 12.0, 4.0) / 16.0);

            self.right_arm.local_matrix = matrix;
            self.right_arm.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;
        }

        {
            let mut matrix = Matrix4::IDENTITY;
            matrix.translatev(Vec3::new(-4.0, 24.0, -4.0) / 16.0);
            matrix.scalev(Vec3::new(8.0, 8.0, 8.0) / 16.0);

            self.head.local_matrix = matrix;
            self.head.texture_idx = GlobalRenderer::WORLD_TEXTURE_IDX as u32;
        }
    }

    pub fn reset(&mut self) {
        self.aabb.set_position(0.0, 60.0, 0.0);
    }

    pub fn cleanup(&mut self) {
        self.selection_box.cleanup();
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs, planet: &mut Planet, particles_manager: &mut ParticlesManager) {
        self.state = if args.current_screen_id == ScreensId::HudScreen {
             PlayerStates::Active
        } else { PlayerStates::Menu };


        let walking = self.process_input(args);
        self.process_collision(args.dt, planet, args.inputs);

        self.in_water = false;
        planet.iterate_over_blocks_cube(&self.aabb, |stop, _, blocks_manager, _, _, _, block_properties| {
            if *block_properties == blocks_manager.water_block {
                self.in_water = true;
                *stop = true;
            }
        });

        if args.inputs.key_pressed(inputs::Keys::F5) {
            match self.camera.get_perspective_type() {
                PerspectiveMode::FirstPerson => self.camera.change_type(PerspectiveMode::ThridPersonBack),
                PerspectiveMode::ThridPersonBack => self.camera.change_type(PerspectiveMode::ThridPersonFront),
                PerspectiveMode::ThridPersonFront => self.camera.change_type(PerspectiveMode::FirstPerson),
            }
        }

        self.camera.update(&self.aabb, planet, args.inputs.get_camera_delta());


        let ray_result = self.update_ray_casting(planet, particles_manager, args.inputs);

        self.selection_box.update(&ray_result);

        let mut action = false;

        if self.state == PlayerStates::Active {
            action = args.inputs.mouse_pressed(inputs::MouseButton::Left);

            self.inventory.process_hotbar_scroll(args.inputs.get_mouse_scroll());

            if let Some(result) = ray_result {
                let slot = self.inventory.get_hand_slot();

                if args.inputs.mouse_pressed(inputs::MouseButton::Right) && let Some(item) = slot.get_item() && item.is_block() {
                    let keep_same_block = result.block_properties.can_replace && item.id != result.block_properties.base_properties.id;
                    let place_block = if keep_same_block { result.block_pos } else { result.block_pos + result.hit_normal };

                    let chunk_pos = math::get_chunk_pos(place_block);
                    let chunk_block = math::get_chunk_block(chunk_pos, place_block);

                    if let Some(chunk) = planet.chunks_manager.get_chunk(chunk_pos) {
                        let ch = chunk.read().unwrap();

                        let block_properties = ch.data.read().unwrap().get_block_properties(chunk_block);

                        if block_properties.can_replace {
                            action = true;

                            planet.place_block(&ch, chunk_block, item.get_id_state());
                        }
                    }
                }
            }
        }

        self.first_person.update(args,
            self.inventory.get_hand_slot(),
            action,
            walking,
            self.velocity,
            if self.state == PlayerStates::Active { Some(args.inputs.get_camera_delta()) } else { None }
        );

        if args.inputs.key_pressed(inputs::Keys::E) {
            args.events_queue.push_back(GameEvents::ChangeScreen(ScreensId::InventoryScreen));
        }
    }

    pub fn draw(&mut self, renderer: &mut EntitiesRenderer, global_renderer: &mut GlobalRenderer) {
        self.selection_box.draw(global_renderer);

        if self.camera.get_perspective_type() == PerspectiveMode::FirstPerson {
            self.first_person.draw(global_renderer);

            return;
        }


        let mut global_pos = self.aabb.get_center();
        global_pos.y = self.aabb.y0;

        let mut global_matrix = Matrix4::IDENTITY;
        global_matrix.translatev(global_pos);
        global_matrix.rotate_xyz(0.0, -self.camera.get_rot().x - 90.0, 0.0);

        let mut head_matrix = Matrix4::IDENTITY;
        head_matrix.translate(0.0, 24.0 / 16.0, 0.0);
        head_matrix.rotate_xyz(self.camera.get_rot().y, 0.0, 0.0);
        head_matrix.translate(0.0, -24.0 / 16.0, 0.0);

        let mut left_leg = self.left_leg.clone();
        let mut right_leg = self.right_leg.clone();
        let mut body = self.body.clone();
        let mut left_arm = self.left_arm.clone();
        let mut right_arm = self.right_arm.clone();
        let mut head = self.head.clone();

        left_leg.local_matrix = global_matrix * left_leg.local_matrix;
        right_leg.local_matrix = global_matrix * right_leg.local_matrix;
        body.local_matrix = global_matrix * body.local_matrix;
        left_arm.local_matrix = global_matrix * left_arm.local_matrix;
        right_arm.local_matrix = global_matrix * right_arm.local_matrix;
        head.local_matrix = global_matrix * head_matrix * head.local_matrix;

        renderer.add_cube(left_leg);
        renderer.add_cube(right_leg);
        renderer.add_cube(body);
        renderer.add_cube(left_arm);
        renderer.add_cube(right_arm);
        renderer.add_cube(head);
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
                let aabb = selection_box.clone_movev(it.global_block);

                if let Some(hit) = aabb.ray_intersect(ray_pos, ray_dir) {
                    // break block
                    if inputs.mouse_pressed(inputs::MouseButton::Left) && self.state == PlayerStates::Active {
                        planet.destroy_block(&it.chunk.read().unwrap(), it.chunk_block, particles_manager);
                    }

                    result = Some(RaycastingResult {
                        block_pos: it.global_block,
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

    fn process_input(&mut self, args: &mut WorldUpdateArgs) -> bool {
        if self.state == PlayerStates::Menu { return false }

        let mut walking = false;
        let mut dir = Vec3::ZERO;

        let forward = self.camera.get_forward();

        if args.inputs.key_down(inputs::Keys::W) { dir = dir + forward; walking = true };
        if args.inputs.key_down(inputs::Keys::A) { dir = dir - forward.cross(Vec3::UP); walking = true };
        if args.inputs.key_down(inputs::Keys::S) { dir = dir - forward; walking = true };
        if args.inputs.key_down(inputs::Keys::D) { dir = dir + forward.cross(Vec3::UP); walking = true };
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

        return walking;
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
            else if !self.in_water {
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
