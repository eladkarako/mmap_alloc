use crate::mmap::MmapMut;
use anyhow::{bail, Context, Result};
use std::{fs::File, mem, os::windows::io::AsRawHandle};

const FILE_END_OF_FILE_INFO_CLASS: u32 = 20u32;

pub fn allocate_file(
    file: &File,
    file_size: u64,
) -> Result<()> {
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;
    if handle as isize == -1 {
        bail!("Invalid file handle");
    }
    if file_size == 0 {
        bail!("Cannot set file information with size 0");
    }
    if file_size > i64::MAX as u64 {
        bail!(
            "Requested file size {} exceeds maximum i64",
            file_size
        );
    }
    let mut info = winapi::um::fileapi::FILE_END_OF_FILE_INFO {
        EndOfFile: unsafe { mem::transmute(file_size as i64) },
    };
    let ret = unsafe {
        winapi::um::fileapi::SetFileInformationByHandle(
            handle,
            FILE_END_OF_FILE_INFO_CLASS,
            &mut info as *mut _ as *mut winapi::ctypes::c_void,
            std::mem::size_of::<
                winapi::um::fileapi::FILE_END_OF_FILE_INFO,
            >() as u32,
        )
    };
    if ret == 0 {
        return Err(std::io::Error::last_os_error()).context(
            format!(
                "SetFileInformationByHandle failed for size {}",
                file_size
            ),
        );
    }
    Ok(())
}
pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}
#[cfg(test)]
mod tests;
