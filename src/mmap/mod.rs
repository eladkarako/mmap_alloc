// C:\Users\Elad\RustroverProjects\mmap_alloc\src\mmap\mod.rs
use anyhow::{bail, Context, Result};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::{
    fs::File,
    ops::{Deref, DerefMut},
    os::windows::io::AsRawHandle,
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
        if len == 0 {
            bail!("Cannot map file with length 0")
        }
        #[cfg(unix)]
        {
            return create_unix_mmap(file, len);
        }
        #[cfg(windows)]
        {
            return create_windows_mmap(file, len);
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
    pub fn flush(&self) -> Result<()> {
        #[cfg(unix)]
        {
            flush_async_unix(self.ptr, self.len)
        }
        #[cfg(windows)]
        {
            flush_sync_windows(self.ptr, self.len)
        }
    }
}
#[cfg(unix)]
fn create_unix_mmap(
    file: &File,
    len: usize,
) -> Result<MmapMut> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        bail!("Invalid file descriptor")
    }
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
            .context("mmap failed");
    }
    Ok(MmapMut { ptr: ptr as *mut u8, len })
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
            .context("msync failed");
    }
    Ok(())
}
#[cfg(windows)]
fn create_windows_mmap(
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
