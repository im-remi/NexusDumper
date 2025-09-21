use crate::ffi::il2cpp_string_new;
use std::ffi::CString;

#[repr(transparent)]
pub struct Il2cppString(pub *const u8);

impl std::fmt::Display for Il2cppString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<usize> for Il2cppString {
    fn from(value: usize) -> Self {
        Self(value as *const u8)
    }
}

impl Il2cppString {
    pub fn new(str: &str) -> Self {
        let c_string = CString::new(str).unwrap();
        let ptr = unsafe { il2cpp_string_new(c_string.as_ptr()) };
        Self(ptr as *const u8)
    }
    pub fn len(&self) -> usize {
        unsafe { *self.0.wrapping_add(16).cast::<u32>() as usize }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            String::from_utf16(std::slice::from_raw_parts(
                self.0.wrapping_add(20).cast::<u16>(),
                self.len(),
            ))
            .unwrap_or_default()
        }
    }
}
