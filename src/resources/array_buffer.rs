use std::ffi::c_void;


pub struct ArrayBuffer<T: Default + Copy, const SIZE: usize> {
    array: [T; SIZE],

    size: usize,
}

impl<T: Default + Copy, const SIZE: usize> ArrayBuffer<T, SIZE> {
    pub fn new() -> Self { Self { array: [Default::default(); SIZE], size: 0 } }

    pub fn clear(&mut self) { self.size = 0 }
    pub fn as_ptr(&self) -> *const c_void { self.array.as_ptr() as *const c_void }
    pub fn len(&self) -> i32 { self.size as i32 }
    pub fn len_bytes(&self) -> isize { (self.size * size_of::<T>()) as isize }

    #[inline]
    pub fn add(&mut self, data: T) {
        if self.size >= self.array.len() {
            panic!("Array buffer capacity exceeded: {}", self.size)
        }

        let temp = self.size;
        self.size += 1;

        self.array[temp] = data;
    }

    pub fn add_range<const SIZE2: usize>(&mut self, other: &ArrayBuffer<T, SIZE2>) {
        let mut items_count = other.size;

        if items_count + self.size > self.array.len() {
            items_count = self.array.len() - self.size
        }

        for index in 0..items_count {
            let last = self.size;
            self.size += 1;

            self.array[last] = other.array[index];
        }
    }
}