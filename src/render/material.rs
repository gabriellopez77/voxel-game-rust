use std::{cell::RefCell, rc::Rc};

use crate::{utils::SafePtrMut};
use super::core::{VulkanApp, GraphicsPipeline};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Sky,
    ChunksOpaque,
    ChunksAlpha,
    Opaque,
    Alpha,
    Particle,
    FirstPerson,
    Ui,
}

pub struct Material {
    app: SafePtrMut<VulkanApp>,

    pub pipeline: Rc<RefCell<GraphicsPipeline>>,

    material_type: MaterialType,
}

unsafe impl Send for Material {}

impl Material {
    pub fn new(app: SafePtrMut<VulkanApp>, pipeline: Rc<RefCell<GraphicsPipeline>>, material_type: MaterialType) -> Self {
        Self {
            app,

            pipeline: pipeline,


            material_type: material_type,
        }
    }

    pub fn destroy(&mut self) {

    }

    pub fn get_type(&self) -> MaterialType { self.material_type }
}
