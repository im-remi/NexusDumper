use std::{borrow::Cow, ffi::{c_void, CStr}, sync::OnceLock};
use windows::{core::s, Win32::System::LibraryLoader::LoadLibraryA};

static ASSEMBLY_BASE: OnceLock<usize> = OnceLock::new();
static UNITY_BASE: OnceLock<usize> = OnceLock::new();

#[inline]
pub unsafe fn cstr(s: *const i8) -> Cow<'static, str> {
    unsafe {
        CStr::from_ptr(s).to_string_lossy()
    }
}

pub fn assembly_base() -> usize {
    *ASSEMBLY_BASE.get_or_init(|| unsafe {
        LoadLibraryA(s!("GameAssembly.dll")).unwrap().0 as usize
    })
}

pub fn unity_base() -> usize {
    *UNITY_BASE.get_or_init(|| unsafe {
        LoadLibraryA(s!("UnityPlayer.dll")).unwrap().0 as usize
    })
}

pub fn unity_ptr(rva: usize) -> *mut c_void {
    unsafe {
        let ptr = *((unity_base() + rva) as *const usize);
        ptr as *mut c_void
    }
}

pub fn assembly_ptr(rva: usize) -> *mut c_void {
    (assembly_base() + rva) as *mut c_void
}

#[macro_export]
macro_rules! as_cstr {
    ($s:expr) => {
        CString::new($s)
            .unwrap()
            .to_bytes_with_nul()
            .as_ptr()
    };
}

#[macro_export]
macro_rules! import {
    ($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty = $rva:expr) => {
        pub unsafe fn $name($($arg: $ty,)*) -> $ret {
            type FuncType = fn($($ty,)*) -> $ret;
            unsafe { ::std::mem::transmute::<*const c_void, FuncType>(unity_ptr($rva))($($arg,)*)}
        }
    };
}


#[macro_export]
macro_rules! import_gameassembly {
    ($name:ident($($arg:ident: $typ:ty),*) -> $ret:ty = $rva:expr) => {
        pub unsafe fn $name($($arg: $typ),*) -> $ret {
            type FuncType = unsafe fn($($typ),*) -> $ret;
            unsafe { ::std::mem::transmute::<*const c_void, FuncType>(assembly_ptr($rva))($($arg,)*)}
        }
    };
}



pub(crate) use as_cstr;
pub(crate) use import_gameassembly;