use std::ops::Deref;


/// safe wrapper around raw ptr that can be null in start, but need be initialized before deref
pub struct NullSafePtr<T: ?Sized> {
    ptr: *const T
}

unsafe impl<T: ?Sized> Send for NullSafePtr<T> {}

impl<T: ?Sized> Deref for NullSafePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null(), "ptr has not initalized!");

        // SAFETY: ptr is not null and valid
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> Clone for NullSafePtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> NullSafePtr<T> {
    pub fn null() -> Self { Self { ptr: unsafe { std::mem::zeroed() } } }

    pub fn new(ptr: &T) -> Self {
        Self { ptr: ptr as *const T }
    }
}
