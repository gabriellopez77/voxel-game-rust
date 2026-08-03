use std::ops::{Deref, DerefMut};


/// safe wrapper around raw ptr not null
pub struct SafePtrMut<T: ?Sized> {
    ptr: *mut T
}

unsafe impl<T: ?Sized> Send for SafePtrMut<T> {}

impl<T: ?Sized> Deref for SafePtrMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr is not null and valid
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> DerefMut for SafePtrMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T: ?Sized> Clone for SafePtrMut<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> SafePtrMut<T> {
    pub fn new(value: &mut T) -> Self {
        Self { ptr: value }
    }

    pub fn from_ptr(ptr: *mut T) -> Self {
        assert!(!ptr.is_null(), "Ptr is null!");

        Self { ptr: ptr }
    }
}
