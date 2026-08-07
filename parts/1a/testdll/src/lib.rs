use std::ptr::null_mut;
use std::thread;

use windows_core::*;
use windows_sys::Win32::{
    Foundation::*,
    Storage::FileSystem::*,
    System::SystemInformation::*,
    UI::WindowsAndMessaging::*,
};

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(
    _dll_module: *mut std::ffi::c_void,
    call_reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> bool {
    const DLL_PROCESS_ATTACH: u32 = 1;
 
    if call_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| {
            unsafe {
                log("DllMain: DLL_PROCESS_ATTACH");
                message_box(w!("DLL_PROCESS_ATTACH"));
            }
        });
    } 
    true
}

#[unsafe(no_mangle)]
pub extern "system" fn Hello(
    _: HWND,
    _: HINSTANCE,
    _: *const u16,
    _: i32,
) {
    unsafe {
        message_box(w!("Hello from test.dll"));
    }
}

unsafe fn message_box(message: PCWSTR) {
    unsafe {
        MessageBoxW(
            GetForegroundWindow(),
            message.0,
            w!("test.dll").0,
            MB_OK | MB_APPLMODAL,
        );
    }
}

unsafe fn log(message: &str) {
    let mut temp_path = [0u16; 261];
    unsafe {
        let len = GetTempPathW(
            temp_path.len() as u32,
            temp_path.as_mut_ptr(),
        );

        if len == 0 || len >= temp_path.len() as u32 {
            return;
        }

        let mut full_path = temp_path[..len as usize].to_vec();

        let name = w!("testdll.log");
        let name_wide = name.as_wide();

        full_path.extend_from_slice(&name_wide[..name_wide.len() - 1]);
        full_path.push(0);

        let h_file = CreateFileW(
            full_path.as_ptr(),
            FILE_GENERIC_WRITE,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        if h_file == INVALID_HANDLE_VALUE {
            return;
        }

        let mut time = SYSTEMTIME {
            wYear: 0,
            wMonth: 0,
            wDayOfWeek: 0,
            wDay: 0,
            wHour: 0,
            wMinute: 0,
            wSecond: 0,
            wMilliseconds: 0,
        };

        GetLocalTime(&mut time);
        let content = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02} {}\r\n",
            time.wYear,
            time.wMonth,
            time.wDay,
            time.wHour,
            time.wMinute,
            time.wSecond,
            message,
        );

        let content = content.as_bytes();
        let mut bytes_written = 0u32;

        SetFilePointer(h_file, 0, null_mut(), FILE_END);
        WriteFile(
            h_file,
            content.as_ptr(),
            content.len() as u32,
            &mut bytes_written,
            null_mut(),
        );

        CloseHandle(h_file);
    }
}
