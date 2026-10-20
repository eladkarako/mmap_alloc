#[cfg(unix)]
use crate::mmap::MmapMut;
use anyhow::{bail, Context};
use std::fs::File;
use std::os::unix::io::AsRawFd;

pub fn create_mmap(
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

pub fn flush_mmap(
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

impl Drop for MmapMut {
    fn drop(&mut self) {
        let _ = unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.len)
        };
    }
}

#[cfg(test)]
mod tests;
