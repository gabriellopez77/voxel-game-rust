pub struct ObjectPool<T> {
    objects: Vec<T>,
}

impl<T> ObjectPool<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn get(&mut self) -> Option<T> {
        if self.objects.len() == 0 { return None }

        return self.objects.pop();
    }

    pub fn restore(&mut self, obj: T) {
        self.objects.push(obj);
    }
}
