//! Memory-mapped file abstraction layer.

use anyhow::Result;
use std::ops::{Deref, DerefMut};

#[cfg(unix)]
use crate::ffi_unix;
#[cfg(windows)]
use crate::ffi_windows;

use anyhow::Context;

/// A mutable memory-mapped file view.
///
/// Provides safe access to memory-mapped file data with platform-specific
/// resource cleanup via RAII.
pub struct MmapMut {
    /// Raw pointer to mapped memory.
    pub ptr: *mut u8,
    /// Length of the mapped region in bytes.
    pub len: usize,
    #[cfg(windows)]
    /// Windows file mapping handle (for cleanup).
    pub file_mapping: crate::bindings::windows::HANDLE,
}

unsafe impl Send for MmapMut {}
unsafe impl Sync for MmapMut {}

impl MmapMut {
    /// Creates a new memory map for the given file.
    ///
    /// # Arguments
    /// * `file` - The opened file to map
    /// * `len` - Number of bytes to map (must be > 0)
    ///
    /// # Errors
    /// Returns error if memory mapping fails.
    #[cfg(unix)]
    pub fn map_mut(file: &std::fs::File, len: usize) -> Result<Self> {
        let ptr = ffi_unix::safe_mmap(file, len)?;
        Ok(MmapMut { ptr, len })
    }

    /// Creates a new memory map for the given file (Windows).
    #[cfg(windows)]
    pub fn map_mut(file: &std::fs::File, len: usize) -> Result<Self> {
        let file_mapping = ffi_windows::safe_create_file_mapping(file, len as u64)?;
        let ptr = ffi_windows::safe_map_view_of_file_ex(file_mapping, len)?;
        Ok(MmapMut { ptr, len, file_mapping })
    }

    /// Returns a mutable slice of the mapped memory.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    /// Returns a raw pointer to the mapped memory.
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    /// Returns the length of the mapped memory.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the mapped region is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Syncs changes to disk synchronously (Unix only).
    #[cfg(unix)]
    pub fn msync_sync(&self) -> Result<()> {
        let ret = unsafe {
            crate::bindings::unix::msync(
                self.ptr as *mut std::ffi::c_void,
                self.len,
                crate::bindings::unix::MS_SYNC,
            )
        };
        if ret != 0 {
            return Err(std::io::Error::last_os_error()).context("msync(2) failed");
        }
        Ok(())
    }

    /// Syncs changes asynchronously (Unix only).
    #[cfg(unix)]
    pub fn msync_async(&self) -> Result<()> {
        let ret = unsafe {
            crate::bindings::unix::msync(
                self.ptr as *mut std::ffi::c_void,
                self.len,
                crate::bindings::unix::MS_ASYNC,
            )
        };
        if ret != 0 {
            return Err(std::io::Error::last_os_error()).context("msync(2) async failed");
        }
        Ok(())
    }

    /// Flushes changes to disk synchronously (Windows only).
    #[cfg(windows)]
    pub fn flush_sync(&self) -> Result<()> {
        let ret = unsafe {
            crate::bindings::windows::FlushViewOfFile(
                self.ptr as *mut std::ffi::c_void,
                self.len,
            )
        };
        if ret == crate::bindings::windows::FALSE {
            return Err(std::io::Error::last_os_error()).context("FlushViewOfFile failed");
        }
        Ok(())
    }

    /// Flushes changes asynchronously (Windows only).
    #[cfg(windows)]
    pub fn flush_async_internal(&self) -> Result<()> {
        let ret = unsafe {
            crate::bindings::windows::FlushViewOfFile(
                self.ptr as *mut std::ffi::c_void,
                self.len,
            )
        };
        if ret == crate::bindings::windows::FALSE {
            return Err(std::io::Error::last_os_error())
                .context("FlushViewOfFile async failed");
        }
        Ok(())
    }
}

impl Deref for MmapMut {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl DerefMut for MmapMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

#[cfg(unix)]
impl Drop for MmapMut {
    fn drop(&mut self) {
        let _ = unix_ffi::safe_munmap(self.ptr, self.len);
    }
}

#[cfg(windows)]
impl Drop for MmapMut {
    fn drop(&mut self) {
        let _ = ffi_windows::safe_unmap_view_of_file(self.ptr);
        let _ = ffi_windows::safe_close_handle(self.file_mapping);
    }
}

#[cfg(test)]
mod tests;
