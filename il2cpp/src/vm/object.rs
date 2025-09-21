use super::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Il2cppObject(pub *const u8);

impl From<usize> for Il2cppObject {
    fn from(value: usize) -> Self {
        Self(value as *const u8)
    }
}

impl From<*const u8> for Il2cppObject {
    fn from(value: *const u8) -> Self {
        Self(value)
    }
}

impl From<Il2cppObject> for usize {
    fn from(value: Il2cppObject) -> Self {
        value.0 as usize
    }
}

impl Il2cppObject {
    pub fn new(class: &Il2cppClass) -> Self {
        unsafe { Self(il2cpp_object_new(class.0)) }
    }

    pub fn class(&self) -> Il2cppClass {
        unsafe { Il2cppClass(*(self.0 as *const usize) as *const u8) }
    }
}
