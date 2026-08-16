use std::*;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
    TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::RemoteDesktop::ProcessIdToSessionId;
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

fn main() {
    let has_filter = env::args().len() > 1;
    let filter = env::args().nth(1);

    let current_pid = unsafe { GetCurrentProcessId() };
    let current_sid = sid_of_pid(current_pid);

    println!("{}", "PID        PARENT     OWNER        NAME");
    println!("{}", "------------------------------------------------------------");

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            println!("{}", "Denied");
            process::exit(1);
        }

        let mut entry: PROCESSENTRY32W = core::mem::zeroed();
        entry.dwSize = core::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..len]);
                if filter.as_deref().is_none_or(|filter| name.to_lowercase().contains(&filter.to_lowercase())) {
                    let pid = entry.th32ProcessID;
                    let parent = entry.th32ParentProcessID;
                    let owner = match sid_of_pid(pid) {
                        Some(0) => "system",
                        Some(sid) if Some(sid) == current_sid => "me",
                        Some(_) => "other",
                        None => "unknown",
                    };

                    print!("{:<11}", pid.to_string());
                    print!("{:<11}", parent.to_string());
                    print!("{:<13}", owner);
                    println!("{}", name);
                }

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
    }

    if !has_filter {
        println!();
        print!("{}", "To filter by name: ");
        print!("{}", env::args().nth(0).unwrap_or_default());
        println!("{}", " [name]");
    }
}

fn sid_of_pid(pid: u32) -> Option<u32> {
    let mut sid = 0u32;
    if unsafe { ProcessIdToSessionId(pid, &mut sid) } != 0 {
        Some(sid)
    } else {
        None
    }
}
