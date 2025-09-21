use super::*;

#[repr(transparent)]
pub struct Il2cppAssembly(*const u8);

impl From<*const u8> for Il2cppAssembly {
    fn from(value: *const u8) -> Self {
        (value as usize != 0)
            .then_some(Self(value))
            .expect("Il2cppAssembly::from(null)")
    }
}

impl Il2cppAssembly {
    pub fn image(&self) -> Il2cppImage {
        unsafe { Il2cppImage::from(il2cpp_assembly_get_image(self.0) as *const u8) }
    }
}
