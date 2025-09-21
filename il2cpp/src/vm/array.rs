use std::mem;

use super::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Il2cppArray(pub *const u8);

impl From<usize> for Il2cppArray {
    fn from(value: usize) -> Self {
        Self(value as *const u8)
    }
}

impl From<Il2cppArray> for usize {
    fn from(value: Il2cppArray) -> Self {
        value.0 as usize
    }
}

impl Il2cppArray {
    pub fn new(array_type: &Il2cppClass, size: usize) -> Self {
        unsafe { Self(il2cpp_array_new(array_type.0, size as u32)) }
    }

    pub fn length(&self) -> usize {
        unsafe { *self.0.wrapping_add(24).cast::<u32>() as usize }
    }

    pub fn data_ptr_raw(&self) -> *const u8 {
        self.0.wrapping_add(32)
    }

    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(mem::transmute(self.0.wrapping_add(32)), self.length())
        }
    }

    pub fn as_mut_slice<T>(&self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(mem::transmute(self.0.wrapping_add(32)), self.length())
        }
    }
}