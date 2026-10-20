use anyhow::{bail, Result};
use std::fs::File;

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
use unix as os_impl;
#[cfg(windows)]
use windows as os_impl;

pub struct MmapMut {
    pub ptr: *mut u8,
    pub len: usize,
    #[cfg(windows)]
    pub file_mapping: *mut winapi::ctypes::c_void,
}

unsafe impl Send for MmapMut {}
unsafe impl Sync for MmapMut {}

impl MmapMut {
    pub fn map_mut(
        file: &File,
        len: usize,
    ) -> Result<Self> {
        if len == 0 {
            bail!("Cannot map file with length 0")
        }
        os_impl::create_mmap(file, len)
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn flush(&self) -> Result<()> {
        os_impl::flush_mmap(self.ptr, self.len)
    }
}

impl std::ops::Deref for MmapMut {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl std::ops::DerefMut for MmapMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
