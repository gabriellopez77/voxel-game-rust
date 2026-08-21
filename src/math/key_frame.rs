use crate::math;


pub struct KeyFrame<T: Copy + Default> {
    frames: Vec<(f32, T)>,
    func: fn (f32, T, T) -> T,

    highest_key: f32,
}

impl<T: Copy + Default> KeyFrame<T> {
    pub fn new(func: fn (f32, T, T) -> T) -> Self {
        Self {
            frames: Vec::new(),
            func,

            highest_key: 0.0,
        }
    }

    pub fn set_frames(&mut self, mut frames: Vec<(f32, T)>) {
        if frames.is_empty() {
            return
        }

        frames.sort_by(|a, b| a.0.total_cmp(&b.0));

        let highest_key = frames.last().unwrap().0;

        for (key, _) in &mut frames {
            if *key < 0.0 {
                *key = 0.0;
            }

            *key = *key / highest_key;
        }

        self.frames = frames;
        self.highest_key = highest_key;
    }

    pub fn get(&self, mut t: f32) -> T {
        if self.frames.is_empty() {
            return T::default()
        }

        t /= self.highest_key;

        let start_value = self.frames[0].1;
        let end_value = self.frames.last().unwrap().1;

        // clamping
        if t <= self.frames[0].0 { return start_value }
        if t >= self.frames.last().unwrap().0 { return end_value }

        // search in range
        for i in 0..self.frames.len() - 1 {
            let current = &self.frames[i];
            let next = &self.frames[i + 1];

            if t >= current.0 && t <= next.0 {
                let mut factor = (t - current.0) / (next.0 - current.0);

                return (self.func)(factor, current.1, next.1);
            }
        }

        unreachable!()
    }
}
