pub struct ObjectPool<T> {
    objects: Vec<T>,
}

impl<T> ObjectPool<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn get(&mut self) -> Option<T> {
        if self.objects.len() == 0 { return None }

        return self.objects.pop();
    }

    pub fn get_or<T2>(&mut self, func: T2) -> T
    where T2: FnOnce() -> T {
        if let Some(obj) = self.objects.pop() {
            return obj;
        }

        return func();
    }

    pub fn restore(&mut self, obj: T) {
        self.objects.push(obj);
    }
}
