use crate::bindings::windows;
use anyhow::{bail, Context, Result};
use std::ffi::c_void;
use std::fs::File;
use std::os::windows::io::AsRawHandle;

/// Safe wrapper for CreateFileMappingW
pub fn safe_create_file_mapping(
    file: &File,
    size: u64,
) -> Result<windows::HANDLE> {
    let handle = file.as_raw_handle() as windows::HANDLE;

    if handle == windows::INVALID_HANDLE_VALUE {
        bail!("Invalid file handle");
    }

    if size == 0 {
        bail!("Cannot create file mapping with size 0");
    }

    let high = ((size >> 32) & 0xFFFFFFFF) as u32;
    let low = (size & 0xFFFFFFFF) as u32;

    let file_mapping = unsafe {
        windows::CreateFileMappingW(
            handle,
            std::ptr::null_mut(),
            windows::PAGE_READWRITE,
            high,
            low,
            std::ptr::null(),
        )
    };

    if file_mapping == windows::INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error())
            .context("CreateFileMappingW failed");
    }

    Ok(file_mapping)
}

/// Safe wrapper for MapViewOfFileEx
pub fn safe_map_view_of_file_ex(
    file_mapping: windows::HANDLE,
    len: usize,
) -> Result<*mut u8> {
    if file_mapping == windows::INVALID_HANDLE_VALUE {
        bail!("Invalid file mapping handle");
    }

    if len == 0 {
        bail!("Cannot map view with length 0");
    }

    let ptr = unsafe {
        windows::MapViewOfFileEx(
            file_mapping,
            windows::FILE_MAP_READ | windows::FILE_MAP_WRITE,
            0,
            0,
            len,
            std::ptr::null_mut(),
        )
    };

    if ptr.is_null() {
        return Err(std::io::Error::last_os_error()).context(
            format!("MapViewOfFileEx failed for {} bytes", len),
        );
    }

    Ok(ptr as *mut u8)
}

/// Safe wrapper for UnmapViewOfFile
pub fn safe_unmap_view_of_file(ptr: *mut u8) -> Result<()> {
    if ptr.is_null() {
        bail!("Cannot unmap null pointer");
    }

    let ret =
        unsafe { windows::UnmapViewOfFile(ptr as *mut c_void) };

    if ret == windows::FALSE {
        return Err(std::io::Error::last_os_error())
            .context("UnmapViewOfFile failed");
    }

    Ok(())
}

/// Safe wrapper for CloseHandle
pub fn safe_close_handle(handle: windows::HANDLE) -> Result<()> {
    if handle == windows::INVALID_HANDLE_VALUE || handle.is_null() {
        bail!("Cannot close invalid or null handle");
    }

    let ret = unsafe { windows::CloseHandle(handle) };

    if ret == windows::FALSE {
        return Err(std::io::Error::last_os_error())
            .context("CloseHandle failed");
    }

    Ok(())
}

/// Safe wrapper for SetFileInformationByHandle
pub fn safe_set_file_information_by_handle(file:&File,file_size:u64,)->Result<()>{
    let handle=file.as_raw_handle()as windows::HANDLE;
    if handle==windows::INVALID_HANDLE_VALUE{
        bail!("Invalid file handle");}
    if file_size==0{
        bail!("Cannot set file information with size 0");}
    if file_size>i64::MAX as u64{
        bail!("Requested file size {} exceeds maximum i64",file_size);}

    let mut info=windows::FILE_END_OF_FILE_INFO{EndOfFile:file_size as i64,};
    let ret=unsafe{windows::SetFileInformationByHandle(
        handle,
        windows::FILE_END_OF_FILE_INFO_CLASS,
        &mut info as*mut _ as*mut c_void,
        std::mem::size_of::<windows::FILE_END_OF_FILE_INFO>()as u32,
    )};
    if ret==windows::FALSE{
        return Err(std::io::Error::last_os_error()).context(format!("SetFileInformationByHandle failed for size {}",file_size),);}
    Ok(())
}


#[cfg(test)]
mod tests;
