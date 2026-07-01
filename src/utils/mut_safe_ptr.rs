use std::ops::{Deref, DerefMut};


/// safe wrapper around raw ptr not null
pub struct MutSafePtr<T: ?Sized> {
    ptr: *mut T
}

impl<T: ?Sized> Deref for MutSafePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr is not null and valid
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> DerefMut for MutSafePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T: ?Sized> Clone for MutSafePtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> MutSafePtr<T> {
    pub fn from(ptr: &mut T) -> Self {
        Self { ptr: ptr }
    }

    pub fn from_ptr(ptr: *mut T) -> Self {
        Self { ptr: ptr }
    }
}
