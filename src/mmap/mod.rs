use anyhow::{bail, Context, Result};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::{
    fs::File,
    ops::{Deref, DerefMut},
};

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
        validate_mmap_length(len)?;

        #[cfg(unix)]
        {
            create_unix_mmap(file, len)
        }

        #[cfg(windows)]
        {
            create_windows_mmap(file, len)
        }
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

    pub fn flush_sync(&self) -> Result<()> {
        #[cfg(unix)]
        {
            flush_sync_unix(self.ptr, self.len)
        }

        #[cfg(windows)]
        {
            flush_sync_windows(self.ptr, self.len)
        }
    }

    pub fn flush_async(&self) -> Result<()> {
        #[cfg(unix)]
        {
            flush_async_unix(self.ptr, self.len)
        }

        #[cfg(windows)]
        {
            self.flush_sync()
        }
    }
}

fn validate_mmap_length(len: usize) -> Result<()> {
    if len == 0 {
        bail!("Cannot map file with length 0");
    }
    Ok(())
}

#[cfg(unix)]
fn create_unix_mmap(
    file: &File,
    len: usize,
) -> Result<MmapMut> {
    let fd = validate_and_extract_file_descriptor(file)?;
    let ptr = create_unix_mmap_pointer(fd, len)?;
    Ok(MmapMut { ptr, len })
}

#[cfg(unix)]
fn validate_and_extract_file_descriptor(file: &File) -> Result<i32> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        bail!("Invalid file descriptor: {}", fd);
    }
    Ok(fd)
}

#[cfg(unix)]
fn create_unix_mmap_pointer(
    fd: i32,
    len: usize,
) -> Result<*mut u8> {
    let ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == libc::MAP_FAILED {
        return Err(std::io::Error::last_os_error())
            .context(format!("mmap(2) failed for {} bytes", len));
    }

    Ok(ptr as *mut u8)
}

#[cfg(unix)]
fn flush_sync_unix(
    ptr: *mut u8,
    len: usize,
) -> Result<()> {
    let ret = unsafe {
        libc::msync(ptr as *mut libc::c_void, len, libc::MS_SYNC)
    };

    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("msync(2) failed");
    }

    Ok(())
}

#[cfg(unix)]
fn flush_async_unix(
    ptr: *mut u8,
    len: usize,
) -> Result<()> {
    let ret = unsafe {
        libc::msync(ptr as *mut libc::c_void, len, libc::MS_ASYNC)
    };

    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("msync(2) async failed");
    }

    Ok(())
}

#[cfg(windows)]
fn create_windows_mmap(
    file: &File,
    len: usize,
) -> Result<MmapMut> {
    let handle = validate_and_extract_windows_handle(file)?;
    let file_mapping = create_windows_file_mapping(handle, len)?;
    let ptr = create_windows_view_of_file(file_mapping, len)?;

    Ok(MmapMut { ptr, len, file_mapping })
}

#[cfg(windows)]
fn validate_and_extract_windows_handle(
    file: &File
) -> Result<*mut winapi::ctypes::c_void> {
    let handle = file.as_raw_handle() as *mut winapi::ctypes::c_void;
    if handle as isize == -1 {
        bail!("Invalid file handle");
    }
    Ok(handle)
}

#[cfg(windows)]
fn create_windows_file_mapping(
    handle: *mut winapi::ctypes::c_void,
    len: usize,
) -> Result<*mut winapi::ctypes::c_void> {
    let (high, low) = split_size_into_high_low_parts(len as u64);

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

    Ok(file_mapping)
}

#[cfg(windows)]
fn split_size_into_high_low_parts(size: u64) -> (u32, u32) {
    let high = ((size) >> 32) as u32;
    let low = (size) as u32;
    (high, low)
}

#[cfg(windows)]
fn create_windows_view_of_file(
    file_mapping: *mut winapi::ctypes::c_void,
    len: usize,
) -> Result<*mut u8> {
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
        return Err(std::io::Error::last_os_error()).context(
            format!("MapViewOfFileEx failed for {} bytes", len),
        );
    }

    Ok(ptr as *mut u8)
}

#[cfg(windows)]
fn flush_sync_windows(
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
        let _ = unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.len)
        };
    }
}

#[cfg(windows)]
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
