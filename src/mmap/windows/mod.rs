#[cfg(windows)]
use crate::mmap::MmapMut;
use anyhow::{bail, Context, Result};
use std::fs::File;
use std::os::windows::io::AsRawHandle;

pub fn create_mmap(
    file: &File,
    len: usize,
) -> Result<MmapMut> {
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;
    if handle as isize == -1 {
        bail!("Invalid file handle")
    }

    let (high, low) =
        ((len as u64 >> 32) as u32, (len as u64) as u32);
    let file_mapping = unsafe {
        winapi::um::memoryapi::CreateFileMappingW(
            handle,
            std::ptr::null_mut(),
            winapi::um::winnt::PAGE_READWRITE,
            high,
            low,
            std::ptr::null(),
        )
    };

    if file_mapping as isize == -1 {
        return Err(std::io::Error::last_os_error())
            .context("CreateFileMappingW failed");
    }

    let ptr = unsafe {
        winapi::um::memoryapi::MapViewOfFileEx(
            file_mapping,
            winapi::um::memoryapi::FILE_MAP_READ
                | winapi::um::memoryapi::FILE_MAP_WRITE,
            0,
            0,
            len,
            std::ptr::null_mut(),
        )
    };

    if ptr.is_null() {
        let _ = unsafe {
            winapi::um::handleapi::CloseHandle(file_mapping)
        };
        return Err(std::io::Error::last_os_error())
            .context("MapViewOfFileEx failed");
    }

    Ok(MmapMut { ptr: ptr as *mut u8, len, file_mapping })
}

pub fn flush_mmap(
    ptr: *mut u8,
    len: usize,
) -> Result<()> {
    let ret = unsafe {
        winapi::um::memoryapi::FlushViewOfFile(
            ptr as *mut winapi::ctypes::c_void,
            len,
        )
    };
    if ret == 0 {
        return Err(std::io::Error::last_os_error())
            .context("FlushViewOfFile failed");
    }
    Ok(())
}

impl Drop for MmapMut {
    fn drop(&mut self) {
        let _ = unsafe {
            winapi::um::memoryapi::UnmapViewOfFile(
                self.ptr as *mut winapi::ctypes::c_void,
            )
        };
        let _ = unsafe {
            winapi::um::handleapi::CloseHandle(self.file_mapping)
        };
    }
}

#[cfg(test)]
mod tests;
