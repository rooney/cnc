use std::env;
use std::ffi::{c_void, OsStr};
use std::io::{self, Read};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

type NtStatus = i32;
type Handle = *mut c_void;

#[repr(C)]
struct UnicodeString {
    length: u16,
    maximum_length: u16,
    buffer: *mut u16,
}

#[repr(C)]
struct ObjectAttributes {
    length: u32,
    root_directory: Handle,
    object_name: *mut UnicodeString,
    attributes: u32,
    security_descriptor: *mut c_void,
    security_quality_of_service: *mut c_void,
}

#[repr(C)]
struct IoStatusBlock {
    status: NtStatus,
    information: usize,
}

unsafe extern "system" {
    fn Sw3NtCreateFile(
        file_handle: *mut Handle,
        desired_access: u32,
        object_attributes: *mut ObjectAttributes,
        io_status_block: *mut IoStatusBlock,
        allocation_size: *mut i64,
        file_attributes: u32,
        share_access: u32,
        create_disposition: u32,
        create_options: u32,
        ea_buffer: *mut c_void,
        ea_length: u32,
    ) -> NtStatus;

    fn Sw3NtWriteFile(
        file_handle: Handle,
        event: Handle,
        apc_routine: *mut c_void,
        apc_context: *mut c_void,
        io_status_block: *mut IoStatusBlock,
        buffer: *const c_void,
        length: u32,
        byte_offset: *mut i64,
        key: *mut u32,
    ) -> NtStatus;

    fn Sw3NtClose(handle: Handle) -> NtStatus;
}

const FILE_APPEND_DATA: u32 = 0x0004;
const FILE_READ_DATA: u32 = 0x0001;

const FILE_SHARE_READ: u32 = 0x0001;
const FILE_SHARE_WRITE: u32 = 0x0002;
const FILE_SHARE_DELETE: u32 = 0x0004;

const FILE_ATTRIBUTE_NORMAL: u32 = 0x0080;

const FILE_OPEN_IF: u32 = 0x00000003;

const FILE_NON_DIRECTORY_FILE: u32 = 0x00000040;
const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x00000020;

const OBJ_CASE_INSENSITIVE: u32 = 0x00000040;

fn nt_success(status: NtStatus) -> bool {
    status >= 0
}

fn nt_error(operation: &str, status: NtStatus) -> io::Error {
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

    let path_len = ((path_wide.len() - 1) * 2) as u16;

    let mut unicode_string = UnicodeString {
        length: path_len,
        maximum_length: path_len + 2,
        buffer: path_wide.as_mut_ptr(),
    };

    let mut object_attributes = ObjectAttributes {
        length: std::mem::size_of::<ObjectAttributes>() as u32,
        root_directory: std::ptr::null_mut(),
        object_name: &mut unicode_string,
        attributes: OBJ_CASE_INSENSITIVE,
        security_descriptor: std::ptr::null_mut(),
        security_quality_of_service: std::ptr::null_mut(),
    };

    let mut file_handle: Handle = std::ptr::null_mut();

    let mut create_status = IoStatusBlock {
        status: 0,
        information: 0,
    };

    let status = unsafe {
        Sw3NtCreateFile(
            &mut file_handle,
            FILE_APPEND_DATA | FILE_READ_DATA,
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

    let mut write_status = IoStatusBlock {
        status: 0,
        information: 0,
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
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "content is too large"))?,
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
