use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    #[link_name = "cheriot_print_stdout"]
    pub fn print_out(str: *const c_char);

    #[link_name = "cheriot_print_stderr"]
    pub fn print_err(str: *const c_char);

    #[link_name = "cheriot_alloc"]
    pub fn alloc(bytes: u32) -> *mut c_void;

    #[link_name = "cheriot_free"]
    pub fn free(ptr: *mut c_void);

    #[link_name = "cheriot_randombytes"]
    pub fn fill_bytes(output: *mut u8, n: u32) -> c_int;
}
