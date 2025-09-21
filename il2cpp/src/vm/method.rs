use super::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct Il2cppMethod(pub *const u8);

impl Il2cppMethod {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_method_get_name(self.0)) }
    }

    pub fn address(&self) -> usize {
        unsafe { *((self.0 as *const usize).offset(2)) }
    }

    pub fn return_type(&self) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_method_get_return_type(self.0).cast()) }
    }

    pub fn param_count(&self) -> u32 {
        unsafe { il2cpp_method_get_param_count(self.0) }
    }

    pub fn param(&self, index: u32) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_method_get_param(self.0, index)) }
    }

    pub fn param_name(&self, index: u32) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_method_get_param_name(self.0, index)) }
    }

    pub fn attrs(&self) -> u32 {
        unsafe { get_method_flags(self.0) as u32 }
    }

    pub fn invoke<T: From<usize>>(
        &self,
        instance: &dyn Il2cppValue,
        args: &[&dyn Il2cppValue],
    ) -> Result<T, Il2cppException> {
        let args = args.iter().map(|arg| arg.as_raw()).collect::<Vec<_>>();

        let mut exception = 0;
        let ret = unsafe {
            il2cpp_runtime_invoke(
                self.0,
                instance.as_raw() as *const u8,
                args.as_ptr() as *const usize,
                &mut exception,
            )
        };
        (exception == 0)
            .then_some(T::from(ret))
            .ok_or(Il2cppException::from(Il2cppObject::from(
                exception as *const u8,
            )))
    }

    pub fn custom_attributes(&self) -> Vec<Il2cppObject> {
        unsafe {
            let attr_info = il2cpp_custom_attrs_from_method(self.0);
            if attr_info.is_null() {
                return Vec::new();
            }

            let attrs_array = il2cpp_custom_attrs_construct(attr_info);

            if attrs_array.is_null() {
                return Vec::new();
            }

            let array = Il2cppArray(attrs_array);
            array
                .as_slice::<*const u8>()
                .iter()
                .filter_map(|&ptr| (!ptr.is_null()).then(|| Il2cppObject(ptr)))
                .collect()
        }
    }
}
