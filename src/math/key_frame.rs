pub struct KeyFrame<T: Copy> {
    frames: Vec<(f32, T)>,
    func: fn (f32, T, T) -> T,

    highest_key: f32,
}

impl<T: Copy> KeyFrame<T> {
    pub fn new(func: fn (f32, T, T) -> T) -> Self {
        Self {
            frames: Vec::new(),
            func,

            highest_key: 0.0,
        }
    }

    pub fn get_highest_key(&self) -> f32 { self.highest_key }
    pub fn get_first_key_frame(&self) -> (f32, T) { self.frames[0] }

    pub fn set_frames(&mut self, mut frames: Vec<(f32, T)>) {
        debug_assert!(!frames.is_empty(), "Frames is empty!");

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
                let factor = (t - current.0) / (next.0 - current.0);

                return (self.func)(factor, current.1, next.1);
            }
        }

        unreachable!()
    }
}
