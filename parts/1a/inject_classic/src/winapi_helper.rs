use windows_sys::Win32::{
    Foundation::*,
    System::{
        Diagnostics::{ Debug::*, ToolHelp::* },
        LibraryLoader::*,
        Memory::*,
        Threading::*,
    },
};

pub fn open_process(pid: u32) -> Result<Process, String> {
    unsafe {
        let process = OpenProcess(
            PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_OPERATION
                | PROCESS_VM_WRITE,
            0,
            pid,
        );
        if process.is_null() {
            return Err(s("OpenProcess failed"));
        }
        Ok(Process(Handle(process)))
    }
}

pub fn get_module_handle(module_name: &str) -> Result<HMODULE, String> {
    unsafe {
        let handle = GetModuleHandleW(w(module_name).as_ptr());
        if handle.is_null() {
            return Err(s("GetModuleHandleW failed"));
        }
        Ok(handle)
    }
}

pub fn get_remote_module_handle(pid: u32, module_name: &str) -> Result<HMODULE, String> {
    let wanted: Vec<u16> = w(module_name);
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(s("CreateToolHelp32Snapshot failed"))
        }
        let _snapshot = Handle(snapshot);
        let mut entry = MODULEENTRY32W {
            dwSize: size_of::<MODULEENTRY32W>() as u32,
            ..core::mem::zeroed()
        };
        let mut next = Module32FirstW(snapshot, &mut entry);
        while next != 0 {
            let len = wcslen(&entry.szModule);
            if &entry.szModule[..len] == wanted.as_slice() {
                return Ok(entry.modBaseAddr as *mut _);
            }
            next = Module32NextW(snapshot, &mut entry);
        }
    }
    Err(format!("Remote module {module_name} not found"))
}

pub fn get_proc_address(module: HMODULE, proc_name: &str) -> Result<usize, String> {
    let Some(address) = (unsafe { GetProcAddress(module, a(proc_name).as_ptr()) }) else {
        return Err(s("GetProcAddress failed"));
    };
    Ok(address as usize)
}

pub struct Process(Handle);

impl Process {
    pub fn virtual_alloc_ex(&self, size: usize) -> Result<RemoteMemory, String> {
        let ptr = unsafe {
            VirtualAllocEx(
                self.0.0,
                core::ptr::null(),
                size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };

        if ptr.is_null() {
            return Err(s("VirtualAllocEx failed"));
        }

        Ok(RemoteMemory { process: self.0.0, ptr, size })
    }

    pub fn create_remote_thread(&self, start_address: usize, parameter: RemoteMemory) -> Result<RemoteThread, String> {
        unsafe {
            let start_address: extern "system" fn(*mut core::ffi::c_void,) -> u32 = 
                core::mem::transmute(start_address);

            let thread = CreateRemoteThread(
                self.0.0,
                core::ptr::null(),
                0,
                Some(start_address),
                parameter.as_ptr(),
                0,
                core::ptr::null_mut(),
            );

            if thread.is_null() {
                return Err(s("CreateRemoteThread failed"));
            }

            Ok(RemoteThread(Handle(thread)))
        }
    }
}

pub struct RemoteMemory {
    process: HANDLE,
    ptr: *mut core::ffi::c_void,
    size: usize,
}

impl RemoteMemory {
    pub fn write_process_memory(&self, data: &[u8]) -> Result<usize, String> {
        debug_assert!(data.len() <= self.size);
        let mut written = 0;
        unsafe {
            if WriteProcessMemory(
                self.process,
                self.ptr,
                data.as_ptr().cast(),
                data.len(),
                &mut written,
            ) == 0 {
                return Err(s("WriteProcessMemory failed"));
            }
        }
        Ok(written)
    }

    pub fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }
}

impl Drop for RemoteMemory {
    fn drop(&mut self) {
        unsafe {
            VirtualFreeEx(self.process, self.ptr, 0, MEM_RELEASE);
        }
    }
}

pub struct RemoteThread(Handle);

impl RemoteThread {
    pub fn wait(&self) -> Result<(), String> {
        unsafe {
            WaitForSingleObject(self.0.0, INFINITE);

            let mut exit_code: u32 = 0;
            if GetExitCodeThread(self.0.0, &mut exit_code) == 0 {
                return Err(s("GetExitCodeThread failed"))
            } else if exit_code == 0 {
                return Err(s("LoadLibraryW returned NULL inside target process. Check dependencies or path permissions."))
            }

            Ok(())
        }
    }
}

struct Handle(HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

pub fn a(s: &str) -> Vec<u8> {
    s.bytes()
        .chain(core::iter::once(0))
        .collect()
}

pub fn w(s: &str) -> Vec<u16> {
    s.encode_utf16()
        .chain(core::iter::once(0))
        .collect()
}

pub fn wcslen(s: &[u16]) -> usize {
    s.iter()
        .position(|&c| c == 0)
        .unwrap_or(s.len())
}

pub fn s(s: &str) -> String {
    s.to_string()
}

