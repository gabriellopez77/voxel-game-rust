use std::{cell::RefCell, rc::Rc};

use crate::{math::{Matrix4, Vec3}, render::{GlobalRenderer, Material, Mesh}, resources::{AnimationFrame, GenericModel, ResourceManager, animation_frame::{AnimationKeyFrameValue, AnimationRunMode, AnimationStatus}}, world::{player::ItemStack, world::WorldUpdateArgs}};


pub struct FirstPerson {
    model_info: Option<(Rc<GenericModel>, Rc<RefCell<Mesh>>)>,
    material: Option<Rc<RefCell<Material>>>,

    last_item_id: u16,

    swap_down_anim: AnimationFrame,
    swap_up_anim: AnimationFrame,

    interact_start_anim: AnimationFrame,
    interact_end_anim: AnimationFrame,

    anim_result: Matrix4,
}

impl FirstPerson {
    pub fn new() -> Self {
        Self {
            model_info: None,
            material: None,

            swap_down_anim: AnimationFrame::new(AnimationRunMode::Once),
            swap_up_anim: AnimationFrame::new(AnimationRunMode::Once),

            interact_start_anim: AnimationFrame::new(AnimationRunMode::Once),
            interact_end_anim: AnimationFrame::new(AnimationRunMode::Once),

            anim_result: Matrix4::IDENTITY,

            last_item_id: 0,
        }
    }



    pub fn start(&mut self, global_renderer: &mut GlobalRenderer, resources: &mut ResourceManager) {
        self.material = Some(global_renderer.get_material("firstPerson"));

        self.swap_down_anim.start(1.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.2, AnimationKeyFrameValue::new(Vec3::new(0.0, -1.3, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);
        self.swap_up_anim.start(1.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, -1.3, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.2, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);


        self.interact_start_anim.start(5.75, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.5, AnimationKeyFrameValue::new(Vec3::new(-0.4, 0.3, 0.0), Vec3::ZERO, Vec3::new(-120.0, 0.0, 0.0))),
            (1.0, AnimationKeyFrameValue::new(Vec3::new(-0.4, -0.75, 0.0), Vec3::ZERO, Vec3::new(-120.0, 0.0, 0.0))),

        ]);

        self.interact_end_anim.start(5.75, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(-0.4, -0.75, 0.0), Vec3::ZERO, Vec3::new(-120.0, 0.0, 0.0))),
            (1.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);

        let hand_model = resources.get_model("playerHand");
        self.model_info = Some((hand_model.clone(), resources.get_or_load_model_mesh("playerHand", &hand_model)));
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs, hand_item: &ItemStack, action: bool) {
        self.anim_result = Matrix4::IDENTITY;

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
                self.anim_result.translatev(result.position);
            }
        }

        if let Some((result, status)) = self.swap_up_anim.update(args.dt) {
            if status == AnimationStatus::Running {
                self.anim_result.translatev(result.position);
            }
        }




        if action {
            self.interact_start_anim.play();
        }

        if let Some((result, status)) = self.interact_start_anim.update(args.dt) {
            if status == AnimationStatus::Finished {
                self.interact_end_anim.play();
            }
            else {
                self.anim_result.translatev(result.position);
                self.anim_result.rotatev_xyz(result.rotation);
            }
        }

        if let Some((result, status)) = self.interact_end_anim.update(args.dt) {
            if status == AnimationStatus::Running {
                self.anim_result.translatev(result.position);
                self.anim_result.rotatev_xyz(result.rotation);
            }
        }
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if let Some((model, mesh)) = &self.model_info {
            //let mut model_matrix = Matrix4::IDENTITY;

            // if self.last_item_id == 0 {
            //    model_matrix.translatev(self.hand_position);
            //    model_matrix.translatev(self.hand_scale * 0.5);
            //    model_matrix.rotatev_xyz(self.hand_rotation);
            //    model_matrix.translatev(self.hand_scale * -0.5);
            //    model_matrix.scalev(self.hand_scale);
            // }
            // else {
            //    model_matrix = model_matrix * model.first_person_display;
            // }

            // if self.last_item_id == 0 {

            //   //model_matrix = model_matrix * self.anim_result * model.first_person_display;
            // }
            // else {

            // }


            let model_matrix = model.first_person_display * self.anim_result;

            global_renderer.set_push_constant(0, &model_matrix);
            global_renderer.draw(&mesh.borrow(), &mut self.material.as_mut().unwrap().borrow_mut());
        }
    }

    pub fn cleanup(&mut self) {

    }
}
