mod array;
mod assembly;
mod attributes;
mod class;
mod domain;
mod exception;
mod field;
mod image;
mod method;
mod object;
mod string;

pub use array::Il2cppArray;
pub use assembly::Il2cppAssembly;
pub use attributes::*;
pub use class::Il2cppClass;
pub use domain::Il2cppDomain;
pub use exception::Il2cppException;
pub use field::Il2cppField;
pub use image::Il2cppImage;
pub use method::Il2cppMethod;
pub use object::Il2cppObject;
pub use string::Il2cppString;

use crate::ffi::*;
use crate::util::*;
use std::borrow::Cow;
use std::ffi::CString;

pub trait Il2cppValue {
    fn as_raw(&self) -> usize;
}

impl<T: Copy + Into<usize>> Il2cppValue for T {
    fn as_raw(&self) -> usize {
        (*self).into()
    }
}

#[repr(transparent)]
pub struct Il2cppType(pub *const u128);
impl Il2cppType {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_type_get_name(self.0)) }
    }

    pub fn attrs(&self) -> u32 {
        unsafe { il2cpp_type_get_attrs(self.0) }
    }

    pub fn is_byref(&self) -> bool {
        unsafe { il2cpp_type_is_byref(self.0) }
    }

    pub fn type_enum(&self) -> u8 {
        unsafe { *self.0.cast::<u8>().wrapping_add(10) }
    }
}
