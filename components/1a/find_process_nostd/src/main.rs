#![no_std]
#![no_main]

use stdlite::*;

lite_main!();
lite_panic_handler!();

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
    TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::RemoteDesktop::ProcessIdToSessionId;
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

fn main() {
    let argv = Args::parse();
    let has_filter = argv.len() > 1;
    let filter = has_filter.then(|| argv.at(1));

    let current_pid = unsafe { GetCurrentProcessId() };
    let current_sid = sid_of_pid(current_pid);

    putln!("PID        PARENT     OWNER        NAME");
    putln!("------------------------------------------------------------");

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            putln!("Denied");
            exit(1);
        }

        let mut entry: PROCESSENTRY32W = core::mem::zeroed();
        entry.dwSize = core::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let name = w!(entry.szExeFile.as_ptr());
                if filter.is_none_or(|filter| name.contains_ignore_case(filter)) {
                    let pid = entry.th32ProcessID;
                    let parent = entry.th32ParentProcessID;
                    let owner = match sid_of_pid(pid) {
                        Some(0) => w!("system"),
                        Some(sid) if Some(sid) == current_sid => w!("me"),
                        Some(_) => w!("other"),
                        None => w!("unknown"),
                    };

                    put_rpad(11, WStringBuf::from_u32(pid));
                    put_rpad(11, WStringBuf::from_u32(parent));
                    put_rpad(13, owner);
                    putln(name);
                }

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
    }

    if !has_filter {
        putln!();
        put!("To filter by name: ");
        put!(argv.at(0));
        putln!(" [name]");
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
