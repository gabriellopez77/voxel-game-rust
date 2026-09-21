use std::{cell::RefCell, rc::Rc};

use crate::{math::{Matrix4, Vec2, Vec3, math}, render::{GlobalRenderer, Material, Mesh}, resources::{AnimationFrame, ItemBlockModel, ResourceManager, animation_frame::{AnimationKeyFrameValue, AnimationRunMode, AnimationStatus}}, world::{blocks::BlockIdState, player::ItemStack, world::WorldUpdateArgs}};


pub struct FirstPerson {
    light_levels: u8,

    item_model_info: Option<(Rc<ItemBlockModel>, Rc<RefCell<Mesh>>)>,
    hand_model_info: Option<(Rc<ItemBlockModel>, Rc<RefCell<Mesh>>)>,

    material: Option<Rc<RefCell<Material>>>,

    swap_down_anim: AnimationFrame,
    swap_up_anim: AnimationFrame,

    interact_anim: AnimationFrame,
    need_play_interact_anim: bool,

    bobbing_anim: AnimationFrame,

    swap_down_anim_result: AnimationKeyFrameValue,
    swap_up_anim_result: AnimationKeyFrameValue,
    interact_anim_result: AnimationKeyFrameValue,

    bobbing_translate: Vec3,

    camera_translate: Vec3,
    idle_translate: Vec3,

    last_id_state: BlockIdState,

    //pub test_pos: Vec3,
}

impl FirstPerson {
    pub fn new() -> Self {
        Self {
            light_levels: 0,

            item_model_info: None,
            hand_model_info: None,

            material: None,

            swap_down_anim: AnimationFrame::new(AnimationRunMode::Once),
            swap_up_anim: AnimationFrame::new(AnimationRunMode::Once),

            interact_anim: AnimationFrame::new(AnimationRunMode::Once),
            need_play_interact_anim: false,

            bobbing_anim: AnimationFrame::new(AnimationRunMode::Once),

            swap_down_anim_result: AnimationKeyFrameValue::default(),
            swap_up_anim_result: AnimationKeyFrameValue::default(),
            interact_anim_result: AnimationKeyFrameValue::default(),

            bobbing_translate: Vec3::ZERO,

            camera_translate: Vec3::ZERO,
            idle_translate: Vec3::ZERO,

            last_id_state: BlockIdState::AIR,

            //test_pos: Vec3::new(0.325, 0.6, 0.05),
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer, resources: &mut ResourceManager) {
        self.swap_down_anim.start(1.0, vec![
            (0.0, None, None, Some(Vec3::ZERO)),
            (0.2, None, None, Some(Vec3::new(-90.0, 0.0, 0.0))),
        ]);
        self.swap_up_anim.start(1.0, vec![
            (0.0, None, None, Some(Vec3::new(-90.0, 0.0, 0.0))),
            (0.2, None, None, Some(Vec3::ZERO)),
        ]);

        self.interact_anim.start(4.0, vec![
            (0.0, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
            (0.5, Some(Vec3::new(-0.14, 0.075, 0.0)), None, Some(Vec3::new(20.0, 55.0, 0.0))),
            (1.0, Some(Vec3::new(-0.14, -0.2, 0.0)), None, None),
            (1.5, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
        ]);

        self.bobbing_anim.start(1.0, vec![
            (0.0, Some(Vec3::new(0.0, 0.0, 0.0)), None, None),
            (0.5, Some(Vec3::new(-0.04, 0.03, 0.0)), None, None),
            (1.0, Some(Vec3::new(0.0, 0.0, 0.0)), None, None),
            (1.5, Some(Vec3::new(0.04, 0.03, 0.0)), None, None),
            (2.0, Some(Vec3::new(0.0, 0.0, 0.0)), None, None),
        ]);

        self.material = Some(global_renderer.get_material("firstPerson"));

        let hand_model = resources.get_model("playerHand");
        self.hand_model_info = Some((hand_model.clone(), resources.get_or_load_model_mesh("playerHand", &hand_model)));
    }

    pub fn update(&mut self,
        args: &mut WorldUpdateArgs,
        hand_item: &ItemStack,
        action: bool,
        walking: bool,
        player_vel: Vec3,
        camera_delta: Vec2,
        light_levels: u8,
    ) {
        self.light_levels = light_levels;

        self.swap_down_anim_result = AnimationKeyFrameValue::default();
        self.swap_up_anim_result = AnimationKeyFrameValue::default();
        self.interact_anim_result = AnimationKeyFrameValue::default();

        let mut item_id_state = BlockIdState::AIR;

        if let Some(item) = hand_item.get_item() {
            item_id_state = item.get_id_state();
        }

        if self.last_id_state != item_id_state {
            self.swap_down_anim.play();
        }
        self.last_id_state = item_id_state;


        if let Some((result, status)) = self.swap_down_anim.update(args.dt) {
            if status == AnimationStatus::Finished {
                if let Some(item) = hand_item.get_item() {
                    let item_model = item.model.clone();
                    self.item_model_info = Some((item_model.clone(), args.resources.get_or_load_model_mesh(item.internal_name, &item_model)));
                }
                else {
                    self.item_model_info = None;
                }

                self.swap_up_anim.play();
            }
            else {
                self.swap_down_anim_result = result;
            }
        }

        if let Some((result, status)) = self.swap_up_anim.update(args.dt) {
            if status == AnimationStatus::Running {
                self.swap_down_anim_result = result;
            }
        }



        if action {
            if self.interact_anim.is_running() {
                self.need_play_interact_anim = true;

                self.interact_anim.speed = 8.0;
            }

            self.interact_anim.play();
        }

        if let Some((result, status)) = self.interact_anim.update(args.dt) {
            if status == AnimationStatus::Finished {
                if self.need_play_interact_anim {
                    self.interact_anim.play();
                    self.interact_anim.speed = 4.0;
                }

                self.need_play_interact_anim = false;
            }
            else {
                self.interact_anim_result = result;
            }
        }



        // camera translate
        if camera_delta != Vec2::ZERO {
            self.camera_translate.x -= (camera_delta.y * 0.05).clamp(-0.25, 0.25);
            self.camera_translate.y += (camera_delta.x * 0.05).clamp(-0.25, 0.25);

            self.camera_translate.x = self.camera_translate.x.clamp(-5.0, 5.0);
            self.camera_translate.y = self.camera_translate.y.clamp(-5.0, 5.0);
        }
        else {
            self.camera_translate.x -= self.camera_translate.x * (math::FRICTION * args.dt);
            self.camera_translate.y -= self.camera_translate.y * (math::FRICTION * args.dt);

            if self.camera_translate.x.abs() < math::EPSILON { self.camera_translate.x = 0.0 }
            if self.camera_translate.y.abs() < math::EPSILON { self.camera_translate.y = 0.0 }
        }


        // walking animation
        let velo_len = (Vec2::new(player_vel.x, player_vel.z).length().abs() * 0.4).min(4.0);
        if walking && velo_len > 0.0 {
            self.bobbing_anim.play();
            self.bobbing_anim.speed = velo_len;
        }
        else {
            self.bobbing_anim.reset();
        }

        if let Some((result, status)) = self.bobbing_anim.update(args.dt) {
            if status == AnimationStatus::Running {
                self.bobbing_translate = result.position;
            }
        }
        else {
            self.bobbing_translate.x -= self.bobbing_translate.x * (math::FRICTION * args.dt);
            self.bobbing_translate.y -= self.bobbing_translate.y * (math::FRICTION * args.dt);

            if self.bobbing_translate.x.abs() < math::EPSILON { self.bobbing_translate.x = 0.0 }
            if self.bobbing_translate.y.abs() < math::EPSILON { self.bobbing_translate.y = 0.0 }
        }

        // idle animation
        self.idle_translate.z = args.time.cos() * args.dt * 50.0;
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        let light_levels = self.light_levels as u32;

        if let Some((hand_model, mesh)) = &self.hand_model_info {
            let mut hand_mat = Matrix4::IDENTITY;
            hand_mat.rotatev_xyz(self.camera_translate);

            //hand_mat.rotatev_xyz(self.idle_translate);

            hand_mat.translatev(hand_model.first_person_display_pos);
            hand_mat.translatev(self.swap_down_anim_result.position);
            hand_mat.translatev(self.swap_up_anim_result.position);
            hand_mat.translatev(self.interact_anim_result.position);
            hand_mat.translatev(self.bobbing_translate);

            let interact_anim_rot_origin = Vec3::new(
                hand_model.first_person_display_scale.x * 0.5,
                hand_model.first_person_display_scale.y * 0.5,
                hand_model.first_person_display_scale.z
            );
            hand_mat.translatev(interact_anim_rot_origin);
            hand_mat.rotatev_xyz(self.swap_down_anim_result.rotation);
            hand_mat.rotatev_xyz(self.swap_up_anim_result.rotation);
            hand_mat.translatev(-interact_anim_rot_origin);

            hand_mat.translatev(hand_model.first_person_display_scale * 0.5);
            hand_mat.rotatev_xyz(self.interact_anim_result.rotation);
            hand_mat.rotatev_xyz(hand_model.first_person_display_rot);
            hand_mat.translatev(hand_model.first_person_display_scale * -0.5);

            hand_mat.scalev(hand_model.first_person_display_scale);


            global_renderer.set_push_constant(0, &hand_mat);
            global_renderer.set_push_constant(size_of::<Matrix4>(), &light_levels);
            global_renderer.draw(&mesh.borrow(), &mut self.material.as_mut().unwrap().borrow_mut());

            if let Some((item_model, mesh)) = &self.item_model_info {
                let mut item_mat = Matrix4::IDENTITY;
                hand_mat.translatev(item_model.first_person_display_pos);

                hand_mat.translatev(item_model.first_person_display_scale * 0.5);
                hand_mat.rotatev_xyz(item_model.first_person_display_rot);
                hand_mat.translatev(item_model.first_person_display_scale * -0.5);

                hand_mat.scalev(item_model.first_person_display_scale);

                item_mat = hand_mat * item_mat;

                global_renderer.set_push_constant(0, &item_mat);
                global_renderer.set_push_constant(size_of::<Matrix4>(), &light_levels);
                global_renderer.draw(&mesh.borrow(), &mut self.material.as_mut().unwrap().borrow_mut());
            }
        }
    }
}
