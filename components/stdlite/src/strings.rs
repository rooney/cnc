use windows_sys::Win32::System::Console::{GetStdHandle, WriteConsoleW, STD_OUTPUT_HANDLE};
use windows_sys::Win32::UI::Shell::StrStrIW;

/// Fixed-capacity NUL-terminated UTF-16 string.
///
/// `N` is the total buffer size, including the terminating NUL.
pub struct WStringBuf<const N: usize> {
    buf: [u16; N],
    len: usize, // excludes the terminating NUL, which is always stored at `buf[len]`
}

impl<const N: usize> WStringBuf<N> {
    /// Creates an empty NUL-terminated UTF-16 string.
    #[must_use]
    pub const fn new() -> Self {
        debug_assert!(N > 0);
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    fn push(&mut self, ch: u16) {
        debug_assert!(self.len + 1 < N);
        self.buf[self.len] = ch;
        self.len += 1;
        self.buf[self.len] = 0;
    }

    #[must_use]
    pub fn as_wstr(&self) -> &WStr {
        unsafe {
            WStr::from_slice_unchecked(&self.buf[..self.len + 1])
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn reverse(&mut self) {
        self.buf[..self.len].reverse();
    }    
}

impl WStringBuf<11> {
    /// Converts a `u32` to its decimal UTF-16 representation.
    #[must_use]
    pub fn from_u32(mut n: u32) -> Self {
        let mut s = Self::new();
        if n == 0 {
            s.push(b'0' as u16);
            return s;
        }

        while n > 0 {
            s.push(b'0' as u16 + (n % 10) as u16);
            n /= 10;
        }

        s.reverse();
        s
    }
}

impl<const N: usize> AsRef<WStr> for WStringBuf<N> {
    fn as_ref(&self) -> &WStr {
        self.as_wstr()
    }
}


#[repr(transparent)]
pub struct WStr([u16]);

impl WStr {
    /// # Safety
    ///
    /// `slice` must be non-empty, and its last element must be the terminating NUL.
    #[must_use]
    pub const unsafe fn from_slice_unchecked(slice: &[u16]) -> &Self {
        unsafe {
            &*(slice as *const [u16] as *const WStr)
        }
    }

    /// # Safety
    ///
    /// `ptr` must point to a valid NUL-terminated UTF-16 string that remains
    /// valid for the lifetime `'a`.
    #[must_use]
    pub unsafe fn from_ptr<'a>(ptr: *const u16) -> &'a Self {
        let mut end = ptr;
        unsafe {
            while *end != 0 {
                end = end.add(1);
            }            
            let slice = core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize + 1);
            Self::from_slice_unchecked(slice)
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[u16] {
        &self.0
    }

    /// # Safety
    ///
    /// `len` is the desired slice length, including the terminating NUL.
    /// It must satisfy `1 <= len <= self.0.len()`, and the last code unit
    /// of the resulting slice must be the terminating NUL.
    #[must_use]
    unsafe fn slice_unchecked(&self, len: usize) -> &WStr {
        unsafe {
            Self::from_slice_unchecked(&self.as_slice()[..len])
        }
    }

    /// Length excluding the terminating NUL.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len() - 1
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn contains_ignore_case(&self, needle: &WStr) -> bool {
        unsafe { !StrStrIW(self.as_ptr(), needle.as_ptr()).is_null() }
    }
}

impl AsRef<WStr> for WStr {
    fn as_ref(&self) -> &WStr {
        self
    }
}

/// Produces a NUL-terminated UTF-16 string slice.
///
/// When passed a string literal, the UTF-16 string is generated at compile time.
///
/// # Safety
///
/// When passed a pointer, it must point to NUL-terminated UTF-16 string.
#[macro_export]
macro_rules! w {
    ($s:literal) => {{
        unsafe {
            $crate::WStr::from_slice_unchecked(
                &::utf16_lit::utf16_null!($s)
            )
        }
    }};

    ($p:expr) => {{
        unsafe { $crate::WStr::from_ptr($p) }
    }};
}


/// Writes NUL-terminated UTF-16 string to the standard output.
#[macro_export]
macro_rules! put {
    ($s:literal) => {
        $crate::put($crate::w!($s))
    };
    ($e:expr) => {
        $crate::put($e)
    };
}

/// Like `put!` but with newline
#[macro_export]
macro_rules! putln {
    () => {
        $crate::putln($crate::w!(""))
    };
    ($s:literal) => {
        $crate::putln($crate::w!($s))
    };
    ($e:expr) => {
        $crate::putln($e)
    };
}

/// `s` must be NUL-terminated
pub fn put(s: impl AsRef<WStr>) {
    let s = s.as_ref();
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        debug_assert!(!handle.is_null());

        let mut written: u32 = 0;        
        let res = WriteConsoleW(
            handle,
            s.as_ptr().cast(),
            s.len() as u32,
            &mut written,
            core::ptr::null_mut(),
        );

        debug_assert_ne!(res, 0);
        debug_assert_eq!(written as usize, s.len());
    }
}

/// Like `put` but with newline
pub fn putln(s: impl AsRef<WStr>) {
    put(s);
    put(crate::w!("\r\n"));
}

const SPACES: &WStr = w!("                ");
const SPACE_CHUNK: usize = SPACES.len();

/// Like `put` but with trailing spaces up to `width`
pub fn put_rpad(width: usize, s: impl AsRef<WStr>) {
    let s = s.as_ref();
    put(s);

    let mut spaces = width.saturating_sub(s.len());
    while spaces >= SPACE_CHUNK {
        put(SPACES);
        spaces -= SPACE_CHUNK;
    }

    put(unsafe { SPACES.slice_unchecked(spaces + 1) });
}
