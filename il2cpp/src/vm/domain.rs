use std::mem;

use super::*;

#[repr(transparent)]
pub struct Il2cppDomain(pub usize);

impl Il2cppDomain {
    pub fn get() -> Self {
        unsafe { Self(il2cpp_domain_get() as usize) }
    }

    pub fn attach_thread(&self) {
        unsafe {
            il2cpp_thread_attach(self.0);
        }
    }

    pub fn assemblies(&self) -> &[Il2cppAssembly] {
        unsafe {
            let mut count = 0;
            let assemblies = il2cpp_domain_get_assemblies(self.0, &mut count);

            std::slice::from_raw_parts(mem::transmute(assemblies), count)
        }
    }

    pub fn assembly_open(&self, name: &str) -> Il2cppAssembly {
        unsafe {
            Il2cppAssembly::from(il2cpp_domain_assembly_open(self.0, as_cstr!(name)) as *const u8)
        }
    }
}
