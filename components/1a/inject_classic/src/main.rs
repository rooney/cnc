mod winapi_helper;
use winapi_helper::*;

use windows_sys::Win32::System::Threading::*;

fn parse_args(args: Vec<String>) -> Result<(u32, Vec<u16>), String> {
    if args.len() != 3 {
        return Err(format!("Usage:\n  {} <pid> <path-to-dll>", &args[0]));
    }

    let Ok(pid) = args[1].parse::<u32>() else {
        return Err(s("Invalid PID"));
    };
    
    let path_to_dll = w(&args[2]);
    Ok((pid, path_to_dll))
}


fn inject(pid: u32, path_to_dll: Vec<u16>) -> Result<(), String> {
    let process = open_process(pid)?;
    let local_kernel32 = get_module_handle("kernel32.dll")?;
    let local_load_library = get_proc_address(local_kernel32, "LoadLibraryW")?;
    let remote_kernel32 = get_remote_module_handle(pid, "kernel32.dll")?;
    let load_library_rva = local_load_library - (local_kernel32 as usize);
    let remote_load_library = (remote_kernel32 as usize) + load_library_rva;
    let remote_mem = process.virtual_alloc_ex(path_to_dll.len() * size_of::<u16>())?;

    let bytes = unsafe {
        core::slice::from_raw_parts(
            path_to_dll.as_ptr().cast::<u8>(),
            path_to_dll.len() * size_of::<u16>(),
        )
    };
    remote_mem.write_process_memory(bytes)?;

    let thread = process.create_remote_thread(remote_load_library, remote_mem)?;
    thread.wait()
}


fn main() -> () {
    let (pid, path_to_dll) = match parse_args(std::env::args().collect()) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{e}");
            unsafe { ExitProcess(2); }
        }
    };

    match inject(pid, path_to_dll) {
        Ok(_) => println!("OK"),
        Err(e) => {
            eprintln!("{e}");
            unsafe { ExitProcess(1); }
        }
    }
}
