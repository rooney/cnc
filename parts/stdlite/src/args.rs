use windows_sys::Win32::Foundation::LocalFree;
use windows_sys::Win32::System::Environment::GetCommandLineW;
use windows_sys::Win32::UI::Shell::CommandLineToArgvW;

/// Command-line arguments parsed using the Windows command-line parser.
pub struct Args {
    argv: *mut *mut u16,
    argc: usize,
}

impl Args {
    #[must_use]
    pub fn parse() -> Self {
        unsafe {
            let mut argc = 0;
            let argv = CommandLineToArgvW(GetCommandLineW(), &mut argc);

            Self {
                argv,
                argc: if argv.is_null() { 0 } else { argc as usize },
            }
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.argc
    }

    /// Returns the argument at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index >= self.len()`.
    #[must_use]
    pub fn at(&self, index: usize) -> &crate::WStr {
        assert!(
            index < self.argc,
            "argument index {} out of bounds (len = {})",
            index,
            self.argc
        );

        crate::w!(*self.argv.add(index))
    }
}

impl Drop for Args {
    fn drop(&mut self) {
        if !self.argv.is_null() {
            unsafe {
                LocalFree(self.argv.cast());
            }
        }
    }
}

