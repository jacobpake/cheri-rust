use crate::alloc::{GlobalAlloc, Layout, System};
use crate::sys::pal::abi;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        unsafe { abi::alloc(size as _) as _ }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            abi::free(ptr as _);
        }
    }
}
