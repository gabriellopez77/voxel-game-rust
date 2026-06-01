use std::ops::{Deref, DerefMut};

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

impl<T: ?Sized> DerefMut for SafePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}

impl<T: ?Sized> Clone for SafePtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> SafePtr<T> {
    pub fn new(ptr: *const T) -> Self {
        if ptr.is_null() {
            panic!("Attempt to create null pointer");
        }

        Self { ptr: ptr as *mut T }
    }

    pub fn from(ptr: &T) -> Self {
        Self { ptr: ptr as *const T }
    }

    pub fn from_mut(ptr: &mut T) -> Self {
        Self { ptr: ptr as *const T }
    }
}