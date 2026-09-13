use serde_json::Value::Object;

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

    pub fn get_from_fn<T2>(&mut self, mut func: T2) -> Option<T>
    where T2: FnMut(&mut T) -> bool
    {
        for i in 0..self.objects.len() {
            if func(&mut self.objects[i]) {
                return Some(self.objects.swap_remove(i));
            }
        }

        return None;
    }
}
