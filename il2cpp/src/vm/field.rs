use super::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct Il2cppField(pub *const u8);

impl Il2cppField {
    pub fn name(&self) -> Cow<'static, str> {
        unsafe {
            cstr(il2cpp_field_get_name(self.0))
        }
    }

    pub fn token(&self) -> u32 {
        unsafe {
            il2cpp_field_get_token(self.0)
        }
    }
    
    pub fn offset(&self) -> u32 {
        unsafe { il2cpp_field_get_offset(self.0) }
    }
    
    pub fn il2cpp_type(&self) -> Il2cppType {
        unsafe {
            Il2cppType(il2cpp_field_get_type(self.0))
        }
    }

    pub fn value(&self, object: &Il2cppObject) -> *const u8 {
        unsafe {
            il2cpp_field_unknown_value(self.0, object.0)
        }
    }

    pub fn static_value(&self) -> usize {
        unsafe {
            let mut out: usize = 0;
            il2cpp_field_static_get_value(self.0, &mut out);
            out
        }
    }

    pub fn custom_attributes(&self) -> Vec<Il2cppObject> {
        unsafe {
            let attr_info = il2cpp_custom_attrs_from_field(self.0);
            if attr_info.is_null() {
                return Vec::new();
            }

            let attrs_array = il2cpp_custom_attrs_construct(attr_info);

            if attrs_array.is_null() {
                return Vec::new();
            }

            let array = Il2cppArray(attrs_array);
            array.as_slice::<*const u8>()
                 .iter()
                 .filter_map(|&ptr| (!ptr.is_null()).then(|| Il2cppObject(ptr)))
                 .collect()
        }
    }

}