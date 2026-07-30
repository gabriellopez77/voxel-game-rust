use std::ops::Deref;


/// safe wrapper around raw ptr not null
pub struct SafePtr<T: ?Sized> {
    ptr: *const T
}

unsafe impl<T: ?Sized> Send for SafePtr<T> {}

impl<T: ?Sized> Deref for SafePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr is not null and has be created from ref (&) or a not null ptr
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> Clone for SafePtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> SafePtr<T> {
    pub fn new(ptr: &T) -> Self {
        Self { ptr: ptr as *const T }
    }

    pub fn from_ptr(ptr: *const T) -> Self {
        assert!(!ptr.is_null(), "ptr is null!");

        Self { ptr }
    }
}
