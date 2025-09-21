use std::ffi::c_void;
use windows::{
    Win32::System::{
        LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
        Memory,
    },
    core::{PCSTR, s},
};

#[inline]
pub unsafe fn is_wine() -> bool {
    const NTDLL: PCSTR = s!("ntdll.dll");
    const WINE_GET_VERSION: PCSTR = s!("wine_get_version");

    unsafe {
        let ntdll = GetModuleHandleA(NTDLL).unwrap();
        GetProcAddress(ntdll, WINE_GET_VERSION).is_some()
    }
}

/// Function to bypass crashes on Linux (also disables memory protection)
pub unsafe fn patch_wintrust() {
    const STUB: [u8; 6] = [0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3];
    const WINTRUST: PCSTR = s!("wintrust.dll");
    const PATCH_FUNCTIONS: [PCSTR; 3] = [
        s!("CryptCATAdminEnumCatalogFromHash"),
        s!("CryptCATCatalogInfoFromContext"),
        s!("CryptCATAdminReleaseCatalogContext"),
    ];

    unsafe {
        let dll = LoadLibraryA(WINTRUST).unwrap();

        for name in PATCH_FUNCTIONS {
            let addr = GetProcAddress(dll, name).unwrap();
            let mut prot = Memory::PAGE_EXECUTE_READWRITE;

            Memory::VirtualProtect(addr as *const c_void, STUB.len(), prot, &mut prot).unwrap();
            std::ptr::copy_nonoverlapping(STUB.as_ptr(), addr as *mut u8, STUB.len());
            Memory::VirtualProtect(addr as *const c_void, STUB.len(), prot, &mut prot).unwrap();
        }
    }
}
