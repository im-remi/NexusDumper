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

        println!("NexusDumper made by Remi <3");

        while GetModuleHandleA(s!("GameAssembly.dll")).is_err() {
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
        let mut dump_cs = File::create("dump/dump.cs").unwrap();
        dumpcs_gen::dump(&mut BufWriter::new(&mut dump_cs)).unwrap();
        
        std::io::stdout().flush().unwrap();
        let mut script_json = File::create("dump/script.json").unwrap();
        println!("Creating script.json...");
        metadata::create_script_json(&mut BufWriter::new(&mut script_json)).unwrap();
        
        std::io::stdout().flush().unwrap();
        let mut cmd_ids_json = File::create("dump/cmd_ids.json").unwrap();
        println!("Extracting CmdIDS...");
        proto_gen::write_cmd_ids(&mut BufWriter::new(&mut cmd_ids_json)).unwrap();
        
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
