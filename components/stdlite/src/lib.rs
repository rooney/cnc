#![no_std]

mod args;
mod mem;
mod nostd;
mod strings;

pub use args::*;
pub use strings::*;

pub fn exit(code: u32) -> ! {
    unsafe { ExitProcess(code) }
}

unsafe extern "system" {
    fn ExitProcess(uExitCode: u32) -> !;
}
