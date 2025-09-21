use std::{
    fs::File,
    io::{BufWriter, Write},
    thread,
    time::Duration,
};

use windows::{
    Win32::{
        Foundation::HINSTANCE,
        System::{Console, LibraryLoader::GetModuleHandleA, SystemServices::DLL_PROCESS_ATTACH},
    },
    core::s,
};

mod util;

unsafe fn thread_fn() {
    unsafe {
        let _ = Console::AllocConsole();

        println!("PararayaDumper made by Remi <3");

        while GetModuleHandleA(s!("GameAssembly.dll")).is_err()
            || GetModuleHandleA(s!("UnityPlayer.dll")).is_err()
        {
            thread::sleep(Duration::from_millis(200));
        }

        while !il2cpp::ffi::il2cpp_is_fully_initialized() {
            std::thread::sleep(Duration::from_millis(100));
        }
        println!("IL2CPP fully initialized, dumping...");

        println!("Generating dump.cs...");
        std::io::stdout().flush().unwrap();
        if !std::fs::exists("dump").unwrap() {
            std::fs::create_dir("dump").unwrap();
        }
        if std::fs::exists("dump/dump.cs").unwrap() {
            std::fs::remove_file("dump/dump.cs").unwrap();
        }
        let mut dump_cs = File::create("dump/dump.cs").unwrap();
        dumpcs_gen::dump(&mut BufWriter::new(&mut dump_cs)).unwrap();

        println!("Done!");
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    unsafe {
        if call_reason == DLL_PROCESS_ATTACH {
            if util::is_wine() {
                util::patch_wintrust()
            }
            thread::spawn(|| thread_fn());
        }
        true
    }
}
