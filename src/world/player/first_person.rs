use std::{cell::RefCell, rc::Rc};

use crate::{math::{Matrix4, Vec2, Vec3}, render::{GlobalRenderer, Material, Mesh}, resources::{AnimationFrame, GenericModel, ResourceManager, animation_frame::{AnimationKeyFrameValue, AnimationRunMode, AnimationStatus}}, world::{player::ItemStack, world::WorldUpdateArgs}};


pub struct FirstPerson {
    model_info: Option<(Rc<GenericModel>, Rc<RefCell<Mesh>>)>,
    material: Option<Rc<RefCell<Material>>>,

    last_item_id: u16,

    swap_down_anim: AnimationFrame,
    swap_up_anim: AnimationFrame,

    interact_start_anim: AnimationFrame,
    need_play_interact_anim: bool,

    //interact_hand_start_anim: AnimationFrame,
    //interact_hand_end_anim: AnimationFrame,

    bobbing_anim: AnimationFrame,


    pub pos: Vec3,
    pub rot: Vec3,

    swap_down_anim_key_frame: AnimationKeyFrameValue,
    swap_up_anim_key_frame: AnimationKeyFrameValue,

    interact_start_key_frame: AnimationKeyFrameValue,
}

impl FirstPerson {
    pub fn new() -> Self {
        Self {
            model_info: None,
            material: None,

            swap_down_anim: AnimationFrame::new(AnimationRunMode::Once),
            swap_up_anim: AnimationFrame::new(AnimationRunMode::Once),

            interact_start_anim: AnimationFrame::new(AnimationRunMode::Once),
            need_play_interact_anim: false,

            //interact_hand_start_anim: AnimationFrame::new(AnimationRunMode::Once),
            //interact_hand_end_anim: AnimationFrame::new(AnimationRunMode::Once),

            bobbing_anim: AnimationFrame::new(AnimationRunMode::Repeat),


            last_item_id: 0,

            pos: Vec3::new(-0.5, 0.175, -0.1),
            rot: Vec3::new(-80.0, 0.0, 45.0),

            swap_down_anim_key_frame: AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
            swap_up_anim_key_frame: AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),

            interact_start_key_frame: AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer, resources: &mut ResourceManager) {
        self.material = Some(global_renderer.get_material("firstPerson"));

        self.swap_down_anim.start(1.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.2, AnimationKeyFrameValue::new(Vec3::new(0.0, -0.5, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);
        self.swap_up_anim.start(1.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, -0.5, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.2, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);


        self.interact_start_anim.start(4.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.5, AnimationKeyFrameValue::new(Vec3::new(-0.5, 0.175, -0.1), Vec3::ZERO, Vec3::new(-80.0, 0.0, 45.0))),
            (0.75, AnimationKeyFrameValue::new(Vec3::new(-0.25, -0.175, -0.05), Vec3::ZERO, Vec3::new(-40.0, 0.0, 22.50))),
            (1.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);

        self.bobbing_anim.start(5.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (1.0, AnimationKeyFrameValue::new(Vec3::new(-0.02, 0.02, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (2.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (3.0, AnimationKeyFrameValue::new(Vec3::new(0.02, 0.02, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (4.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);

        let hand_model = resources.get_model("playerHand");
        self.model_info = Some((hand_model.clone(), resources.get_or_load_model_mesh("playerHand", &hand_model)));
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs, hand_item: &ItemStack, action: bool, player_vel: Vec3) {
        self.swap_down_anim_key_frame = AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
        self.swap_up_anim_key_frame = AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
        self.interact_start_key_frame = AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);

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
                self.swap_up_anim.play();
            }
            else {
                self.swap_down_anim_key_frame = result;
            }
        }

        if let Some((result, status)) = self.swap_up_anim.update(args.dt) {
            if status == AnimationStatus::Running {
                self.swap_down_anim_key_frame = result;
            }
        }


        //self.interact_start_anim.start(1.0, vec![
        //    (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        //    (0.5, AnimationKeyFrameValue::new(self.pos, Vec3::ZERO, self.rot)),
        //    (1.0, AnimationKeyFrameValue::new(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO)),

        //]);



        if action {
            if self.interact_start_anim.is_running() {
                self.need_play_interact_anim = true;

                self.interact_start_anim.speed *= 3.0;
            }

            self.interact_start_anim.play();
        }

        if let Some((result, status)) = self.interact_start_anim.update(args.dt) {
            if status == AnimationStatus::Finished {
                if self.need_play_interact_anim {
                    self.interact_start_anim.play();
                    self.interact_start_anim.speed /= 3.0;
                }

                self.need_play_interact_anim = false;
            }
            else {
                self.interact_start_key_frame = result;
            }
        }

        //self.bobbing_anim.speed = (Vec2::new(player_vel.x, player_vel.z).length() * 1.5).min(8.0);
        //if let Some((result, _)) = self.bobbing_anim.update(args.dt) {
        //    self.parent_result.translatev(result.position);
        //}
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if let Some((model, mesh)) = &self.model_info {
            let mut model_matrix = Matrix4::IDENTITY;
            model_matrix.translatev(model.pos);
            model_matrix.translatev(self.swap_down_anim_key_frame.position);
            model_matrix.translatev(self.swap_up_anim_key_frame.position);
            model_matrix.translatev(self.interact_start_key_frame.position);

            model_matrix.translatev(model.scale * 0.5);
            model_matrix.rotatev_xyz(self.interact_start_key_frame.rotation);
            model_matrix.translatev(model.scale * -0.5);

            model_matrix.translatev(model.scale * 0.5);
            model_matrix.rotatev_xyz(model.rot);
            model_matrix.translatev(model.scale * -0.5);

            model_matrix.scalev(model.scale);

            global_renderer.set_push_constant(0, &model_matrix);
            global_renderer.draw(&mesh.borrow(), &mut self.material.as_mut().unwrap().borrow_mut());
        }
    }
}
