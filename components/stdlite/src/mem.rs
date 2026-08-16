use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_ZERO_MEMORY,
};

/// The Windows process heap guarantees at least 16-byte alignment on x86-64.
const MAX_SUPPORTED_ALIGN: usize = 16;

struct WinHeap;

unsafe impl GlobalAlloc for WinHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        unsafe {
            HeapAlloc(
                GetProcessHeap(),
                0,
                layout.size().max(1),
            ) as *mut u8
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.align() > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        unsafe {
            HeapAlloc(
                GetProcessHeap(),
                HEAP_ZERO_MEMORY,
                layout.size().max(1),
            ) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        unsafe {
            HeapFree(GetProcessHeap(), 0, ptr.cast());
        }
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        if layout.align() > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        unsafe {
            HeapReAlloc(
                GetProcessHeap(),
                0,
                ptr.cast(),
                new_size.max(1),
            ) as *mut u8
        }
    }
}

#[global_allocator]
static ALLOCATOR: WinHeap = WinHeap;
