use super::*;
use crate::vm::field::Il2cppField;
use std::ffi::c_void;

#[repr(transparent)]
#[derive(Clone)]
pub struct Il2cppClass(pub *const u8);

impl From<*const u8> for Il2cppClass {
    fn from(value: *const u8) -> Self {
        (value as usize != 0)
            .then_some(Self(value))
            .expect("Il2cppClass::from(null)")
    }
}

impl Il2cppClass {
    pub fn token(&self) -> u32 {
        unsafe { il2cpp_class_get_token(self.0) }
    }

    pub fn name(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_class_get_name(self.0)) }
    }

    pub fn namespace(&self) -> Cow<'static, str> {
        unsafe { cstr(il2cpp_class_get_namespace(self.0)) }
    }

    pub fn is_enum(&self) -> bool {
        unsafe { il2cpp_class_is_enum(self.0) }
    }

    pub fn is_struct(&self) -> bool {
        unsafe { il2cpp_class_is_valuetype(self.0) }
    }

    pub fn parent_class(&self) -> Option<Self> {
        unsafe {
            let ptr = il2cpp_class_get_parent(self.0);
            ((ptr as usize) != 0).then_some(Self(ptr))
        }
    }

    pub fn flags(&self) -> u32 {
        unsafe { il2cpp_class_get_flags(self.0) }
    }

    pub fn interfaces(&self) -> Vec<Il2cppClass> {
        unsafe {
            let mut interfaces = Vec::new();
            let mut interface_iter = std::ptr::null();
            loop {
                let interface = il2cpp_class_get_interfaces(self.0, &mut interface_iter);
                if interface.is_null() {
                    break;
                }
                interfaces.push(Il2cppClass(interface));
            }
            interfaces
        }
    }

    pub fn fields(&self) -> Vec<Il2cppField> {
        let mut fields = Vec::new();
        let mut field_iter: *const c_void = std::ptr::null();

        unsafe {
            loop {
                let result = il2cpp_class_get_fields(self.0, &mut field_iter);
                if result.is_null() {
                    break;
                }
                fields.push(Il2cppField(result));
            }
        }
        fields
    }

    pub fn get_field(&self, name: &str) -> Option<Il2cppField> {
        for field in self.fields() {
            if field.name() == name {
                return Some(field);
            }
        }
        None
    }

    pub fn methods(&self) -> Vec<Il2cppMethod> {
        let mut methods = Vec::new();
        let mut method_iter: *const c_void = std::ptr::null();

        unsafe {
            loop {
                let result = il2cpp_class_get_methods(self.0, &mut method_iter);
                if result.is_null() {
                    break;
                }
                methods.push(Il2cppMethod(result));
            }
        }
        methods
    }

    pub fn il2cpp_type(&self) -> Il2cppType {
        unsafe { Il2cppType(il2cpp_class_get_type(self.0)) }
    }

    pub fn get_method(&self, name: &str, param_count: usize) -> Option<Il2cppMethod> {
        for method in self.methods() {
            if method.name() == name && method.param_count() == param_count as u32 {
                return Some(method);
            }
        }
        None
    }

    pub fn get_method_recursive(&self, name: &str, param_count: usize) -> Option<Il2cppMethod> {
        let mut current = Some(self.clone());
        while let Some(c) = current {
            if let Some(m) = c.get_method(name, param_count) {
                return Some(m);
            }
            let parent_class = c.parent_class();
            current = parent_class;
        }
        None
    }

    pub fn custom_attributes(&self) -> Vec<Il2cppObject> {
        unsafe {
            let attr_info = il2cpp_custom_attrs_from_class(self.0);
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
