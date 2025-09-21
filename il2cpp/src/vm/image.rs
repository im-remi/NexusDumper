use super::*;

#[repr(transparent)]
pub struct Il2cppImage(pub *const u8);

impl From<*const u8> for Il2cppImage {
    fn from(value: *const u8) -> Self {
        (value as usize != 0)
            .then_some(Self(value))
            .expect("Il2cppImage::from(null)")
    }
}

impl Il2cppImage {
    pub fn class_count(&self) -> usize {
        unsafe { il2cpp_image_get_class_count(self.0) }
    }

    pub fn get_class(&self, index: usize) -> Il2cppClass {
        unsafe { Il2cppClass::from(il2cpp_image_get_class(self.0, index) as *const u8) }
    }

    pub fn get_class_from_name(&self, namespace: &str, name: &str) -> Il2cppClass {
        unsafe {
            Il2cppClass::from(il2cpp_class_from_name(
                self.0,
                as_cstr!(namespace) as *const i8,
                as_cstr!(name) as *const i8,
            ))
        }
    }

    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_image_get_name(self.0)) }
    }
}
