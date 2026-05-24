pub struct ObjectPool<T> {
    objects: Vec<T>,
}

impl<T> ObjectPool<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn count(&self) -> usize { self.objects.len() }

    pub fn get(&mut self) -> Option<T> {
        if self.objects.len() == 0 { return None }

        return self.objects.pop();
    }

    pub fn insert(&mut self, obj: T) {
        self.objects.push(obj);
    }
}
