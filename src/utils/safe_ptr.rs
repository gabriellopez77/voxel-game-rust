use std::ops::Deref;


/// safe wrapper around raw ptr not null
pub struct SafePtr<T: ?Sized> {
    ptr: *const T
}

impl<T: ?Sized> Deref for SafePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr is not null and valid
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> Clone for SafePtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> SafePtr<T> {
    pub fn from(ptr: &T) -> Self {
        Self { ptr: ptr as *const T }
    }
}
