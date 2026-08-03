use std::{cell::RefCell, rc::Rc};

use crate::{math::{Matrix4, Vec3}, render::{GlobalRenderer, Material, Mesh, material::MaterialType}, resources::{self, AnimationFrame, ResourceManager, animation_frame::{AnimationKeyFrameValue, AnimationRunMode, AnimationStatus}}, world::{player::ItemStack, world::WorldUpdateArgs}};


pub struct FirstPerson {
    mesh: Option<Rc<RefCell<Mesh>>>,
    material: Option<Material>,

    visible: bool,

    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub swap_down_anim: AnimationFrame,
    pub swap_up_anim: AnimationFrame,
    swap_anim_result: Matrix4,

    last_item_id: u16,
}

impl FirstPerson {
    pub fn new() -> Self {
        Self {
            mesh: None,
            material: None,

            visible: false,

            position: Vec3::new(0.4, -0.67, -1.0),
            rotation: Vec3::new(0.0, 45.0, 0.0),
            scale: Vec3::new(0.375, 0.375, 0.375),

            swap_down_anim: AnimationFrame::new(AnimationRunMode::Once),
            swap_up_anim: AnimationFrame::new(AnimationRunMode::Once),
            swap_anim_result: Matrix4::IDENTITY,

            last_item_id: 0,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        self.material = Some(global_renderer.create_material("firstPerson", MaterialType::FirstPerson));

        self.swap_down_anim.start(1.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.2, AnimationKeyFrameValue::new(Vec3::new(0.0, -0.5, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);
        self.swap_up_anim.start(1.0, vec![
            (0.0, AnimationKeyFrameValue::new(Vec3::new(0.0, -0.5, 0.0), Vec3::ZERO, Vec3::ZERO)),
            (0.2, AnimationKeyFrameValue::new(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, Vec3::ZERO)),
        ]);
    }

    pub fn update(&mut self, args: &mut WorldUpdateArgs, hand_item: &ItemStack) {
        self.swap_anim_result = Matrix4::IDENTITY;

        if let Some(item) = hand_item.get_item() {
            self.visible = true;
            
            if self.last_item_id != item.id {
                self.swap_down_anim.play();
            }
            
            if let Some((result, status)) = self.swap_down_anim.update(args.dt) {
                if status == AnimationStatus::Finished {
                    if let Some(item) = hand_item.get_item() {
                        self.mesh = Some(args.resources.get_or_load_model_mesh(item.internal_name, &item.mesh));
                    }
                    self.swap_up_anim.play();
                }
                else {
                    self.swap_anim_result.translatev(result.position);
                }
            }
    
            if let Some((result, status)) = self.swap_up_anim.update(args.dt) {
                if status == AnimationStatus::Running {
                    self.swap_anim_result.translatev(result.position);
                }
            }
            
            self.last_item_id = item.id;
        }
        else {
            self.visible = false;            
        }
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if !self.visible { return }

        if let Some(ref mesh) = self.mesh {
            let mut model = Matrix4::IDENTITY;
            model = model * self.swap_anim_result;

            model.translatev(self.position);
            model.translatev(self.scale * 0.5);
            model.rotatev_xyz(self.rotation);
            model.translatev(self.scale * -0.5);
            model.scalev(self.scale);

            global_renderer.set_push_constant(0, &model);
            global_renderer.draw(&mesh.borrow(), self.material.as_ref().unwrap());
        }
    }

    pub fn cleanup(&mut self) {
        self.material.as_mut().unwrap().destroy();
    }
}
