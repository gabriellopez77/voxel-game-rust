pub struct KeyFrame<T: Copy> {
    pub frames: Vec<(f32, T)>,
    func: fn (f32, T, T) -> T,
}

impl<T: Copy> KeyFrame<T> {
    pub fn new(func: fn (f32, T, T) -> T) -> Self {
        Self {
            frames: Vec::new(),
            func,
        }
    }

    pub fn get(&self, t: f32) -> T {
        let start_value = self.frames[self.frames.len() - 1].1;
        let end_value = self.frames[0].1;

        // clamping
        if t <= self.frames[0].0 { return start_value }
        if t >= self.frames[self.frames.len() - 1].0 { return end_value }

        // search in range
        for i in 0..self.frames.len() - 1 {
            let current = &self.frames[i];
            let next = &self.frames[i + 1];

            if t >= current.0 && t <= next.0 {
                let factor = (t - current.0) / (next.0 - current.0);

                return (self.func)(factor, current.1, next.1);
            }
        }

        return end_value;
    }
}
