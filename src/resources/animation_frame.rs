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

#[derive(Clone, Copy)]
pub struct AnimationKeyFrameValue {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl AnimationKeyFrameValue {
    pub fn new(position: Vec3, scale: Vec3, rotation: Vec3,) -> Self {
        Self { position, scale, rotation }
    }
}

pub struct AnimationFrame {
    key_frames: KeyFrame<AnimationKeyFrameValue>,

    run_mode: AnimationRunMode,

    time: f32,
    pub speed: f32,

    run: bool,
}

impl AnimationFrame {
    pub fn new(run_mode: AnimationRunMode) -> Self {
        Self {
            key_frames: KeyFrame::new(|key, a, b| {
                let position = Vec3::lerp(a.position, b.position, key);
                let scale = Vec3::lerp(a.scale, b.scale, key);
                let rotation = Vec3::lerp(a.rotation, b.rotation, key);

                return AnimationKeyFrameValue { position, scale, rotation };
            }),

            run_mode,
            time: 0.0,
            speed: 1.0,
            run: false,
        }
    }

    pub fn start(&mut self, speed: f32, frames: Vec<(f32, AnimationKeyFrameValue)>) {
        self.key_frames.set_frames(frames);
        self.speed = speed;
    }

    pub fn play(&mut self) {
        if !self.run {
            self.time = 0.0;
            self.run = true;
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<(AnimationKeyFrameValue, AnimationStatus)> {
        if !self.run {
            return None;
        }

        self.time += dt * self.speed;

        let mut finished = false;
    
        if self.time >= self.key_frames.get_highest_key() {
            self.run = false;
            finished = true;

            if self.run_mode == AnimationRunMode::Repeat {
                self.time = 0.0;
                self.run = true;
            }
        }

        let value = self.key_frames.get(self.time);

        return Some((value, if finished { AnimationStatus::Finished } else { AnimationStatus::Running }));
    }
}
