#[expect(dead_code)]
#[path = "unsupported.rs"]
mod unsupported_stdio;

use crate::ffi::CString;
use crate::io::{self};

unsafe extern "C" {
    fn cheriot_print_str_stdout(v: *const core::ffi::c_char);
    fn cheriot_print_str_stderr(v: *const core::ffi::c_char);
}

pub type Stdin = unsupported_stdio::Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let cstr = CString::new(buf).unwrap();

        unsafe {
            cheriot_print_str_stdout(cstr.as_ptr());
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let cstr = CString::new(buf).unwrap();

        unsafe {
            cheriot_print_str_stderr(cstr.as_ptr());
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = unsupported_stdio::STDIN_BUF_SIZE;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
