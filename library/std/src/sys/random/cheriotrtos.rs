use crate::sys::pal::abi;

pub fn fill_bytes(bytes: &mut [u8]) {
    unsafe {
        abi::fill_bytes(bytes.as_mut_ptr(), bytes.len() as _);
    }
}
