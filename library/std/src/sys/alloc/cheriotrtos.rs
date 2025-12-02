use crate::alloc::{GlobalAlloc, Layout, System};

// TODO(jacobpake): we can use ffi here, see
#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}

    unsafe fn realloc(&self, _ptr: *mut u8, _layout: Layout, _new_size: usize) -> *mut u8 {
        core::ptr::null_mut()
    }
}
