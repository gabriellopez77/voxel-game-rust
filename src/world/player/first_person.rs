use std::{cell::RefCell, rc::Rc};

use crate::{math::{Matrix4, Vec2, Vec3, math}, render::{GlobalRenderer, Material, Mesh}, resources::{AnimationFrame, GenericModel, ResourceManager, animation_frame::{AnimationKeyFrameValue, AnimationRunMode, AnimationStatus}}, world::{player::ItemStack, world::WorldUpdateArgs}};


pub struct FirstPerson {
    model_info: Option<(Rc<GenericModel>, Rc<RefCell<Mesh>>)>,
    material: Option<Rc<RefCell<Material>>>,

    swap_down_anim: AnimationFrame,
    swap_up_anim: AnimationFrame,

    interact_hand_anim: AnimationFrame,
    interact_anim: AnimationFrame,
    need_play_interact_anim: bool,

    bobbing_anim: AnimationFrame,

    swap_down_anim_result: AnimationKeyFrameValue,
    swap_up_anim_result: AnimationKeyFrameValue,
    interact_anim_result: AnimationKeyFrameValue,

    bobbing_translate: Vec3,

    camera_translate: Vec3,
    idle_translate: Vec3,

    last_item_id: u16,
    is_hand_model: bool,
}

impl FirstPerson {
    pub fn new() -> Self {
        Self {
            model_info: None,
            material: None,

            swap_down_anim: AnimationFrame::new(AnimationRunMode::Once),
            swap_up_anim: AnimationFrame::new(AnimationRunMode::Once),

            interact_hand_anim: AnimationFrame::new(AnimationRunMode::Once),
            interact_anim: AnimationFrame::new(AnimationRunMode::Once),
            need_play_interact_anim: false,

            bobbing_anim: AnimationFrame::new(AnimationRunMode::Once),

            swap_down_anim_result: AnimationKeyFrameValue::default(),
            swap_up_anim_result: AnimationKeyFrameValue::default(),
            interact_anim_result: AnimationKeyFrameValue::default(),

            bobbing_translate: Vec3::ZERO,

            camera_translate: Vec3::ZERO,
            idle_translate: Vec3::ZERO,

            last_item_id: 0,
            is_hand_model: true,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer, resources: &mut ResourceManager) {
        self.material = Some(global_renderer.get_material("firstPerson"));

        self.swap_down_anim.start(1.0, vec![
            (0.0, Some(Vec3::ZERO), None, None),
            (0.2, Some(Vec3::new(0.0, -0.5, 0.0)), None, None),
        ]);
        self.swap_up_anim.start(1.0, vec![
            (0.0, Some(Vec3::new(0.0, -0.5, 0.0)), None, None),
            (0.2, Some(Vec3::ZERO), None, None),
        ]);


        self.interact_anim.start(4.0, vec![
            (0.0, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
            (0.5, Some(Vec3::new(-0.5, 0.175, -0.1)), None, Some(Vec3::new(-80.0, 0.0, 45.0))),
            (1.0, Some(Vec3::new(-0.25, -0.2, -0.05)), None, None),
            (1.5, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
        ]);

        self.interact_hand_anim.start(4.0, vec![
            (0.0, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
            (0.5, Some(Vec3::new(-0.17, 0.1, 0.0)), None, Some(Vec3::new(0.0, 70.0, 0.0))),
            (1.0, Some(Vec3::new(-0.17, -0.2, 0.0)), None, None),
            (1.5, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
        ]);

        self.bobbing_anim.start(1.0, vec![
            (0.0, Some(Vec3::new(0.0, 0.0, 0.0)), None, None),
            (0.5, Some(Vec3::new(-0.04, 0.03, 0.0)), None, None),
            (1.0, Some(Vec3::new(0.0, 0.0, 0.0)), None, None),
            (1.5, Some(Vec3::new(0.04, 0.03, 0.0)), None, None),
            (2.0, Some(Vec3::new(0.0, 0.0, 0.0)), None, None),
        ]);

        let hand_model = resources.get_model("playerHand");
        self.model_info = Some((hand_model.clone(), resources.get_or_load_model_mesh("playerHand", &hand_model)));
    }

    pub fn update(&mut self,
        args: &mut WorldUpdateArgs,
        hand_item: &ItemStack,
        action: bool,
        walking: bool,
        player_vel: Vec3,
        camera_delta: Option<Vec2>,
    ) {
        self.swap_down_anim_result = AnimationKeyFrameValue::default();
        self.swap_up_anim_result = AnimationKeyFrameValue::default();
        self.interact_anim_result = AnimationKeyFrameValue::default();

        let model: Rc<GenericModel>;
        let item_id: u16;
        let model_name: &'static str;

        if let Some(item) = hand_item.get_item() {
            model = item.model.clone();
            model_name = item.internal_name;
            item_id = item.id;
        }
        else {
            model = args.resources.get_model("playerHand");
            model_name = "playerHand";
            item_id = 0;
        }

        if self.last_item_id != item_id {
            self.swap_down_anim.play();
        }
        self.last_item_id = item_id;


        if let Some((result, status)) = self.swap_down_anim.update(args.dt) {
            if status == AnimationStatus::Finished {
                self.model_info = Some((model.clone(), args.resources.get_or_load_model_mesh(model_name, &model)));
                self.is_hand_model = item_id == 0;
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


        //self.interact_hand_anim.start(1.0, vec![
        //    (0.0, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
        //    (0.5, Some(self.pos), None, Some(self.rot)),
        //    (0.75, Some(Vec3::new(-0.1, -0.1, 0.0)), None, None),
        //    (1.0, Some(Vec3::ZERO), None, Some(Vec3::ZERO)),
        //]);


        let interact_anim = if self.is_hand_model {
            &mut self.interact_hand_anim
        }
        else { &mut self.interact_anim };

        if action {
            if interact_anim.is_running() {
                self.need_play_interact_anim = true;

                interact_anim.speed = 8.0;
            }

            interact_anim.play();
        }

        if let Some((result, status)) = interact_anim.update(args.dt) {
            if status == AnimationStatus::Finished {
                if self.need_play_interact_anim {
                    interact_anim.play();
                    interact_anim.speed = 4.0;
                }

                self.need_play_interact_anim = false;
            }
            else {
                self.interact_anim_result = result;
            }
        }



        // camera translate
        if let Some(camera_delta) = camera_delta && camera_delta != Vec2::ZERO {
            self.camera_translate.x += (camera_delta.y * 0.01).clamp(-0.25, 0.25);
            self.camera_translate.y += (camera_delta.x * 0.01).clamp(-0.25, 0.25);

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
        //self.idle_translate.x = args.time.sin() * args.dt * 10.0;
        self.idle_translate.z = args.time.cos() * args.dt * 50.0;
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if let Some((model, mesh)) = &self.model_info {
            let mut model_matrix = Matrix4::IDENTITY;
            model_matrix.rotatev_xyz(self.camera_translate);

            model_matrix.rotatev_xyz(self.idle_translate);

            model_matrix.translatev(model.first_person_display_pos);
            model_matrix.translatev(self.swap_down_anim_result.position);
            model_matrix.translatev(self.swap_up_anim_result.position);
            model_matrix.translatev(self.interact_anim_result.position);
            model_matrix.translatev(self.bobbing_translate);

            model_matrix.translatev(model.first_person_display_scale * 0.5);
            model_matrix.rotatev_xyz(self.interact_anim_result.rotation);
            model_matrix.translatev(model.first_person_display_scale * -0.5);

            model_matrix.translatev(model.first_person_display_scale * 0.5);
            model_matrix.rotatev_xyz(model.first_person_display_rot);
            model_matrix.translatev(model.first_person_display_scale * -0.5);

            model_matrix.scalev(model.first_person_display_scale);

            global_renderer.set_push_constant(0, &model_matrix);
            global_renderer.draw(&mesh.borrow(), &mut self.material.as_mut().unwrap().borrow_mut());
        }
    }
}
