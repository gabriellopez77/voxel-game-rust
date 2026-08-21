use std::f32;

use crate::math::{KeyFrame, Vec3};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationRunMode {
    Repeat,
    Once,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationStatus {
    Finished,
    Running,
}

#[derive(Clone, Copy, Default)]
pub struct AnimationKeyFrameValue {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl AnimationKeyFrameValue {
    pub fn new(position: Vec3, scale: Vec3, rotation: Vec3) -> Self {
        Self {
            position,
            scale,
            rotation
        }
    }
}

pub struct AnimationFrame {
    position_frames: KeyFrame<Vec3>,
    scale_frames: KeyFrame<Vec3>,
    rotation_frames: KeyFrame<Vec3>,

    run_mode: AnimationRunMode,

    pub speed: f32,

    highest_key: f32,
    time: f32,
    run: bool,
}

impl AnimationFrame {
    pub fn new(run_mode: AnimationRunMode) -> Self {
        Self {
            position_frames: KeyFrame::new(|key, a, b| Vec3::lerp(a, b, key)),
            scale_frames: KeyFrame::new(|key, a, b| Vec3::lerp(a, b, key)),
            rotation_frames: KeyFrame::new(|key, a, b| Vec3::lerp(a, b, key)),

            run_mode,

            speed: 1.0,

            highest_key: 0.0,
            time: 0.0,
            run: false,
        }
    }

    pub fn is_running(&self) -> bool { self.run }

    pub fn start(&mut self, speed: f32, frames: Vec<(f32, Option<Vec3>, Option<Vec3>, Option<Vec3>)>) {
        let mut positions = Vec::<(f32, Vec3)>::new();
        let mut scales = Vec::<(f32, Vec3)>::new();
        let mut rotations = Vec::<(f32, Vec3)>::new();

        let mut highest_key = f32::MIN;

        for (key, position, scale, rotation) in &frames {
            highest_key = highest_key.max(*key);

            if let Some(position) = position {
                positions.push((*key, *position));
            }

            if let Some(scale) = scale {
                scales.push((*key, *scale));
            }

            if let Some(rotation) = rotation {
                rotations.push((*key, *rotation));
            }
        }

        self.speed = speed;
        self.highest_key = highest_key;

        self.position_frames.set_frames(positions);
        self.scale_frames.set_frames(scales);
        self.rotation_frames.set_frames(rotations);
    }

    pub fn play(&mut self) {
        if !self.run {
            self.time = 0.0;
            self.run = true;
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
        self.run = false;
    }

    pub fn update(&mut self, dt: f32) -> Option<(AnimationKeyFrameValue, AnimationStatus)> {
        if !self.run && self.run_mode != AnimationRunMode::Repeat {
            return None;
        }

        self.time += dt * self.speed;

        let mut finished = false;

        if self.time >= self.highest_key {
            self.run = false;
            finished = true;

            if self.run_mode == AnimationRunMode::Repeat {
                self.time = 0.0;
                self.run = true;
            }
        }

        let position = self.position_frames.get(self.time);
        let scale = self.scale_frames.get(self.time);
        let rotation = self.rotation_frames.get(self.time);

        let key_frame = AnimationKeyFrameValue { position, scale, rotation };

        return Some((key_frame, if finished { AnimationStatus::Finished } else { AnimationStatus::Running }));
    }
}
