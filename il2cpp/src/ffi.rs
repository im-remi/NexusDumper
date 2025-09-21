use crate::*;
use std::ffi::c_void;

use crate::util::{import_gameassembly};

import_gameassembly!(il2cpp_array_new(ty: *const u8, size: u32) -> *const u8 = 0x1547DE0);

import_gameassembly!(il2cpp_assembly_get_image(assembly: *const u8) -> *const c_void = 0x1547E20);

import_gameassembly!(il2cpp_class_from_name(image: *const u8, namespace: *const i8, name: *const i8) -> *const u8 = 0x1547E90);
import_gameassembly!(il2cpp_class_get_fields(class: *const u8, iter: *const *const c_void) -> *const u8 = 0x1547F30);
import_gameassembly!(il2cpp_class_get_interfaces(class: *const u8, iter: *const *const c_void) -> *const u8 = 0x1547F60);
import_gameassembly!(il2cpp_class_get_methods(class: *const u8, iter: *const *const c_void) -> *const MethodInfo = 0x1548110);
import_gameassembly!(il2cpp_class_get_name(class: *const u8) -> *const i8 = 0x1548120);
import_gameassembly!(il2cpp_class_get_namespace(class: *const u8) -> *const i8 = 0x1548140);
import_gameassembly!(il2cpp_class_get_parent(class: *const u8) -> *const u8 = 0x1548170);
import_gameassembly!(il2cpp_class_is_valuetype(class: *const u8) -> bool = 0x15482B0);
import_gameassembly!(il2cpp_class_is_blittable(class: *const u8) -> bool = 0x1548280);
import_gameassembly!(il2cpp_class_get_flags(class: *const u8) -> u32 = 0x1547F40);
import_gameassembly!(il2cpp_class_is_enum(class: *const u8) -> bool = 0x1548240);

import_gameassembly!(il2cpp_domain_get() -> *const c_void = 0x15483E0);
import_gameassembly!(il2cpp_domain_assembly_open(domain: usize, name: *const u8) -> usize = 0x15483E0);
import_gameassembly!(il2cpp_domain_get_assemblies(domain: usize, size: &mut usize) -> *const c_void = 0x15483F0);

import_gameassembly!(il2cpp_field_get_flags(field: *const u8) -> i32 = 0x1548430);
import_gameassembly!(il2cpp_field_get_name(field: *const u8) -> *const i8 = 0x1548440);
import_gameassembly!(il2cpp_field_get_offset(field: *const u8) -> u32 = 0x1548460);
import_gameassembly!(il2cpp_field_get_type(field: *const u8) -> *const u128 = 0x1548480);
import_gameassembly!(il2cpp_field_unknown_value(field: *const u8, object: *const u8) -> *const u8 = 0x15484A0);
import_gameassembly!(il2cpp_field_static_get_value(field: *const u8, value: *mut usize) -> () = 0x1548500);

import_gameassembly!(il2cpp_method_get_return_type(method: *const MethodInfo) -> *const u8 = 0x15496B0);
import_gameassembly!(il2cpp_method_get_name(method: *const MethodInfo) -> *const i8 = 0x1549640);
import_gameassembly!(il2cpp_method_get_param_count(method: *const MethodInfo) -> u32 = 0x15629C0);
import_gameassembly!(il2cpp_method_get_param(method: *const MethodInfo, index: u32) -> *const u128 = 0x1562950);
import_gameassembly!(il2cpp_method_get_param_name(method: *const MethodInfo, index: u32) -> *const i8 = 0x1549690);
import_gameassembly!(il2cpp_method_get_flags(class: *const MethodInfo) -> u32 = 0x1549600);


import_gameassembly!(il2cpp_object_new(class: *const u8) -> *const u8 = 0x1549870);

import_gameassembly!(il2cpp_runtime_invoke(method: *const MethodInfo, obj: *const u8, params: *const usize, exception: &mut usize) -> usize = 0x1549960);

import_gameassembly!(il2cpp_string_new(ptr: *const i8) -> usize = 0x1549DB0);

import_gameassembly!(il2cpp_thread_attach(domain: usize) -> usize = 0x1549DF0);

import_gameassembly!(il2cpp_type_get_name(ty: *const u128) -> *const i8 = 0x154A000);
import_gameassembly!(il2cpp_type_is_byref(ty: *const u128) -> bool = 0x154A0B0);
import_gameassembly!(il2cpp_type_get_attrs(ty: *const u128) -> u32 = 0x1549F20);

import_gameassembly!(il2cpp_image_get_name(image: *const u8) -> *const i8 = 0x1549500);

import_gameassembly!(il2cpp_image_get_class_count(image: *const u8) -> usize = 0x15494E0);
import_gameassembly!(il2cpp_image_get_class(image: *const u8, index: usize) -> *const c_void = 0x15494D0);

import_gameassembly!(il2cpp_custom_attrs_from_class(klass: *const u8) -> *const u8 = 0x1548350);
import_gameassembly!(il2cpp_custom_attrs_from_method(method: *const MethodInfo) -> *const u8 = 0x1548390);
import_gameassembly!(il2cpp_custom_attrs_from_field(klass: *const u8) -> *const u8 = 0x1548360);

import_gameassembly!(il2cpp_custom_attrs_get_attr(ainfo: *const u8, attr_klass: *const u8) -> *const u8 = 0x15483A0);
import_gameassembly!(il2cpp_custom_attrs_has_attr(ainfo: *const u8, attr_klass: *const u8) -> bool = 0x15483A0);
import_gameassembly!(il2cpp_custom_attrs_construct(cinfo: *const u8) -> *const u8 = 0x1548340);

import_gameassembly!(il2cpp_field_get_token(field: *const u8) -> u32 = 0x15E3AD0);

pub unsafe fn il2cpp_class_get_type(class: *const u8) -> *const u128 {
    class.wrapping_add(56).cast::<u128>() 
}

pub unsafe fn get_method_flags(method: *const MethodInfo) -> u16 {
    unsafe {
        *(method.cast::<u8>().add(56) as *const u16) 
    }
}


pub unsafe fn il2cpp_class_get_token(class: *const u8) -> u32 {
    unsafe {
        *(class.cast::<u32>().add(61) as *const u32)
    }
}


pub fn il2cpp_is_fully_initialized() -> bool {
    const G_IL2CPP_IS_FULLY_INITIALIZED: usize = 0x545EA08;

    unsafe {
        *(util::assembly_base().wrapping_add(G_IL2CPP_IS_FULLY_INITIALIZED) as *const u8) != 0
    }
}


#[repr(C)]
pub struct MethodInfo {
    pub invoker_method: *const c_void,   // 0x00
    pub method_pointer: *const c_void,   // 0x08
    _pad: [u8; 0x20],                    // 0x10..0x40
    pub flags: u16,                      // 0x40 (64 decimal)
}
