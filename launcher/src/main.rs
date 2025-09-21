#![allow(clippy::missing_transmute_annotations)]

use std::ffi::CString;
use std::process::ExitCode;
use std::ptr::null_mut;
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx,
};
use windows::Win32::System::Threading::{
    CREATE_SUSPENDED, CreateProcessA, CreateRemoteThread, PROCESS_INFORMATION, ResumeThread,
    STARTUPINFOA, WaitForSingleObject,
};
use windows::core::{PCSTR, PSTR, s};

const EXECUTABLES: &[&str] = &["NexusAnima.exe"];
const DLLS: &[&str] = &["parayaya.dll"];

fn main() -> ExitCode {
    let current_dir = std::env::current_dir().unwrap();

    for &exe_name in EXECUTABLES {
        if current_dir.join(exe_name).is_file() {
            eprintln!("Found game executable: {exe_name}");
            let exe_name = CString::new(exe_name).unwrap();
            let mut proc_info = PROCESS_INFORMATION::default();
            let startup_info = STARTUPINFOA::default();

            unsafe {
                CreateProcessA(
                    PCSTR(exe_name.as_bytes_with_nul().as_ptr()),
                    PSTR(null_mut()),
                    None,
                    None,
                    false,
                    CREATE_SUSPENDED,
                    None,
                    None,
                    &startup_info,
                    &mut proc_info,
                )
                .unwrap();

                let all_injected = DLLS.iter().all(|dll_name| {
                    let dll_path = current_dir.join(dll_name);
                    if !dll_path.is_file() {
                        eprintln!("DLL not found: {dll_name}");
                        return false;
                    }
                    inject_standard(proc_info.hProcess, dll_path.to_str().unwrap())
                });

                if all_injected {
                    ResumeThread(proc_info.hThread);
                } else {
                    eprintln!("One or more DLLs failed to inject.");
                }

                CloseHandle(proc_info.hThread).unwrap();
                CloseHandle(proc_info.hProcess).unwrap();
            }

            return ExitCode::SUCCESS;
        }
    }

    eprintln!("can't find game executable in this directory");
    let _ = std::io::stdin().read_line(&mut String::new());

    ExitCode::FAILURE
}

fn inject_standard(h_target: HANDLE, dll_path: &str) -> bool {
    unsafe {
        let loadlib = GetProcAddress(
            GetModuleHandleA(s!("kernel32.dll")).unwrap(),
            s!("LoadLibraryA"),
        )
        .unwrap();

        let dll_path_cstr = CString::new(dll_path).unwrap();
        let dll_path_addr = VirtualAllocEx(
            h_target,
            None,
            dll_path_cstr.to_bytes_with_nul().len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        if dll_path_addr.is_null() {
            println!("VirtualAllocEx failed. Last error: {:?}", GetLastError());
            return false;
        }

        WriteProcessMemory(
            h_target,
            dll_path_addr,
            dll_path_cstr.as_ptr() as _,
            dll_path_cstr.to_bytes_with_nul().len(),
            None,
        )
        .unwrap();

        let h_thread = CreateRemoteThread(
            h_target,
            None,
            0,
            Some(std::mem::transmute(loadlib)),
            Some(dll_path_addr),
            0,
            None,
        )
        .unwrap();

        WaitForSingleObject(h_thread, 0xFFFFFFFF);

        VirtualFreeEx(h_target, dll_path_addr, 0, MEM_RELEASE).unwrap();
        CloseHandle(h_thread).unwrap();
        true
    }
}
