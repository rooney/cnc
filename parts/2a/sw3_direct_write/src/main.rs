use std::env;
use std::ffi::{c_void, OsStr};
use std::io::{self, Read};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use ntapi::ntioapi::{
    FILE_NON_DIRECTORY_FILE,
    FILE_OPEN_IF,
    FILE_SYNCHRONOUS_IO_NONALERT,
    IO_STATUS_BLOCK,
};

use winapi::shared::ntdef::{
    HANDLE,
    NTSTATUS,
    OBJECT_ATTRIBUTES,
    UNICODE_STRING,
    OBJ_CASE_INSENSITIVE,
};

use winapi::um::winnt::{
    FILE_APPEND_DATA,
    FILE_ATTRIBUTE_NORMAL,
    FILE_READ_DATA,
    FILE_SHARE_DELETE,
    FILE_SHARE_READ,
    FILE_SHARE_WRITE,
    SYNCHRONIZE,
};
unsafe extern "system" {
    fn Sw3NtCreateFile(
        file_handle: *mut HANDLE,
        desired_access: u32,
        object_attributes: *mut OBJECT_ATTRIBUTES,
        io_status_block: *mut IO_STATUS_BLOCK,
        allocation_size: *mut i64,
        file_attributes: u32,
        share_access: u32,
        create_disposition: u32,
        create_options: u32,
        ea_buffer: *mut c_void,
        ea_length: u32,
    ) -> NTSTATUS;

    fn Sw3NtWriteFile(
        file_handle: HANDLE,
        event: HANDLE,
        apc_routine: *mut c_void,
        apc_context: *mut c_void,
        io_status_block: *mut IO_STATUS_BLOCK,
        buffer: *const c_void,
        length: u32,
        byte_offset: *mut i64,
        key: *mut u32,
    ) -> NTSTATUS;

    fn Sw3NtClose(handle: HANDLE) -> NTSTATUS;
}

fn nt_success(status: NTSTATUS) -> bool {
    status >= 0
}

fn nt_error(operation: &str, status: NTSTATUS) -> io::Error {
    io::Error::other(format!("{operation} failed: NTSTATUS 0x{status:08X}"))
}

fn read_content(args: &[String]) -> io::Result<Vec<u8>> {
    if let Some(content) = args.get(2) {
        return Ok(content.as_bytes().to_vec());
    }

    let mut content = Vec::new();
    io::stdin().read_to_end(&mut content)?;
    Ok(content)
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("usage: {} <filename> [content]", args[0]);
        eprintln!("       {} <filename> < input", args[0]);
        std::process::exit(2);
    }

    let path = PathBuf::from(&args[1]);
    let path = if path.is_absolute() {
        path
    } else {
        env::current_dir()?.join(path)
    };

    let content = read_content(&args)?;

    let nt_path = format!(r"\??\{}", path.display());

    let mut path_wide: Vec<u16> = OsStr::new(&nt_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let path_length = ((path_wide.len() - 1) * 2) as u16;

    let mut unicode_string = UNICODE_STRING {
        Length: path_length,
        MaximumLength: path_length + 2,
        Buffer: path_wide.as_mut_ptr(),
    };

    let mut object_attributes = OBJECT_ATTRIBUTES {
        Length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
        RootDirectory: std::ptr::null_mut(),
        ObjectName: &mut unicode_string,
        Attributes: OBJ_CASE_INSENSITIVE,
        SecurityDescriptor: std::ptr::null_mut(),
        SecurityQualityOfService: std::ptr::null_mut(),
    };

    let mut file_handle: HANDLE = std::ptr::null_mut();

    let mut create_status = IO_STATUS_BLOCK {
        u: unsafe { std::mem::zeroed() },
        Information: 0,
    };

    let status = unsafe {
        Sw3NtCreateFile(
            &mut file_handle,
            FILE_APPEND_DATA | FILE_READ_DATA | SYNCHRONIZE,
            &mut object_attributes,
            &mut create_status,
            std::ptr::null_mut(),
            FILE_ATTRIBUTE_NORMAL,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            FILE_OPEN_IF,
            FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT,
            std::ptr::null_mut(),
            0,
        )
    };

    if !nt_success(status) {
        return Err(nt_error("NtCreateFile", status));
    }

    let mut write_status = IO_STATUS_BLOCK {
        u: unsafe { std::mem::zeroed() },
        Information: 0,
    };

    let write_status_code = unsafe {
        Sw3NtWriteFile(
            file_handle,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut write_status,
            content.as_ptr() as *const c_void,
            u32::try_from(content.len())
                .map_err(|_| io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "content is too large",
                ))?,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    let close_status = unsafe { Sw3NtClose(file_handle) };

    if !nt_success(write_status_code) {
        return Err(nt_error("NtWriteFile", write_status_code));
    }

    if !nt_success(close_status) {
        return Err(nt_error("NtClose", close_status));
    }

    Ok(())
}
