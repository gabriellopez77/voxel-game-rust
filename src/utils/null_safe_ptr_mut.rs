use std::ops::Deref;
use std::ops::DerefMut;


/// safe wrapper around raw ptr that can be null in start, but need be initialized before deref
pub struct NullSafePtrMut<T: ?Sized> {
    ptr: *mut T
}

unsafe impl<T: ?Sized> Send for NullSafePtrMut<T> {}

impl<T: ?Sized> DerefMut for NullSafePtrMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null(), "ptr has not initalized!");

        // SAFETY: ptr is not null and valid
        unsafe { &mut *self.ptr }
    }
}

impl<T: ?Sized> Deref for NullSafePtrMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null(), "ptr has not initalized!");

        // SAFETY: ptr is not null and valid
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> Clone for NullSafePtrMut<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> NullSafePtrMut<T> {
    pub fn null() -> Self { Self { ptr: unsafe { std::mem::zeroed() } } }

    pub fn new(ptr: &mut T) -> Self {
        Self { ptr: ptr as *mut T }
    }
}
