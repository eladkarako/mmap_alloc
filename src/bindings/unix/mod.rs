use std::ffi::c_void;

pub type c_int = i32;
pub type off_t = i64;
pub type size_t = usize;

pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const MAP_SHARED: c_int = 1;
pub const MAP_FAILED: *mut c_void = -1isize as *mut c_void;
pub const MS_SYNC: c_int = 4;
pub const MS_ASYNC: c_int = 1;

#[link(name = "c")]
extern "C" {
    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;

    pub fn munmap(
        addr: *mut c_void,
        len: size_t,
    ) -> c_int;

    pub fn fallocate(
        fd: c_int,
        mode: c_int,
        offset: off_t,
        len: off_t,
    ) -> c_int;

    pub fn madvise(
        addr: *mut c_void,
        len: size_t,
        advice: c_int,
    ) -> c_int;

    pub fn msync(
        addr: *mut c_void,
        len: size_t,
        flags: c_int,
    ) -> c_int;
}
