use std::ops::Deref;
use crate::world::blocks::BlockFunctions;


#[derive(Copy, Clone)]
pub struct BlocksWrapper {
    ptr: *const dyn BlockFunctions,
}

impl Deref for BlocksWrapper {
    type Target = dyn BlockFunctions;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr has be created from & then it is valid
        unsafe { &*self.ptr }
    }
}

impl BlocksWrapper {
    pub fn new(ptr: *const dyn BlockFunctions) -> Self {
        Self { ptr, }
    }
}

