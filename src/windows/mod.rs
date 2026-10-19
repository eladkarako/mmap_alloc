// C:\Users\Elad\RustroverProjects\mmap_alloc\src\windows\mod.rs
#[cfg(windows)]
use crate::mmap::MmapMut;
use anyhow::{bail, Context, Result};
use std::{fs::File, os::windows::io::AsRawHandle};
pub fn allocate_file(
    file: &File,
    file_size: u64,
) -> Result<()> {
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;
    if handle as isize == -1 {
        bail!("Invalid file handle")
    }
    if file_size == 0 {
        bail!("Cannot allocate size 0")
    }
    if file_size > i64::MAX as u64 {
        bail!("Size exceeds i64::MAX")
    }
    let (high, low) = ((file_size >> 32) as u32, file_size as u32);
    let ret = unsafe {
        winapi::um::fileapi::SetFilePointer(
            handle,
            low as i32,
            &mut (high as i32),
            2,
        )
    };
    if ret == 0xFFFFFFFF {
        return Err(std::io::Error::last_os_error())
            .context("SetFilePointer failed");
    }
    let ret = unsafe { winapi::um::fileapi::SetEndOfFile(handle) };
    if ret == 0 {
        return Err(std::io::Error::last_os_error())
            .context("SetEndOfFile failed");
    }
    Ok(())
}
pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}
