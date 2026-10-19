use crate::mmap::MmapMut;
use anyhow::{bail, Context, Result};
use std::{fs::File, mem, os::windows::io::AsRawHandle};

const FILE_END_OF_FILE_INFO_CLASS: u32 = 20u32;

pub fn allocate_file(
    file: &File,
    file_size: u64,
) -> Result<()> {
    validate_file_allocation_request(file, file_size)?;
    set_windows_file_end_of_file(file, file_size)
}

pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}

fn validate_file_allocation_request(
    file: &File,
    file_size: u64,
) -> Result<()> {
    validate_windows_file_handle(file)?;
    validate_file_size_is_nonzero(file_size)?;
    validate_file_size_within_limits(file_size)?;
    Ok(())
}

fn validate_windows_file_handle(file: &File) -> Result<()> {
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;
    if handle as isize == -1 {
        bail!("Invalid file handle");
    }
    Ok(())
}

fn validate_file_size_is_nonzero(file_size: u64) -> Result<()> {
    if file_size == 0 {
        bail!("Cannot set file information with size 0");
    }
    Ok(())
}

fn validate_file_size_within_limits(file_size: u64) -> Result<()> {
    if file_size > i64::MAX as u64 {
        bail!(
            "Requested file size {} exceeds maximum i64",
            file_size
        );
    }
    Ok(())
}

fn set_windows_file_end_of_file(
    file: &File,
    file_size: u64,
) -> Result<()> {
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;

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

#[cfg(test)]
mod tests;
