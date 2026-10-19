use crate::bindings::unix;
use anyhow::{bail, Context};
use std::ffi::c_void;
use std::fs::File;
use std::os::unix::io::AsRawFd;

/// Safe wrapper for fallocate(2)
pub fn safe_fallocate(
    file: &File,
    size: u64,
) -> Result<()> {
    let fd = file.as_raw_fd();

    if fd < 0 {
        bail!("Invalid file descriptor: {}", fd);
    }

    if size == 0 {
        bail!("Cannot allocate file with size 0");
    }

    if size > i64::MAX as u64 {
        bail!(
            "Requested allocation size {} exceeds maximum i64",
            size
        );
    }

    let ret =
        unsafe { unix::fallocate(fd, 0, 0, size as unix::off_t) };

    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("fallocate(2) failed");
    }

    Ok(())
}

/// Safe wrapper for mmap(2)
pub fn safe_mmap(
    file: &File,
    len: usize,
) -> Result<*mut u8> {
    let fd = file.as_raw_fd();

    if fd < 0 {
        bail!("Invalid file descriptor: {}", fd);
    }

    if len == 0 {
        bail!("Cannot map file with length 0");
    }

    let ptr = unsafe {
        unix::mmap(
            std::ptr::null_mut(),
            len,
            unix::PROT_READ | unix::PROT_WRITE,
            unix::MAP_SHARED,
            fd,
            0,
        )
    };

    if ptr == unix::MAP_FAILED {
        return Err(std::io::Error::last_os_error())
            .context(format!("mmap(2) failed for {} bytes", len));
    }

    Ok(ptr as *mut u8)
}

/// Safe wrapper for munmap(2)
pub fn safe_munmap(
    ptr: *mut u8,
    len: usize,
) -> Result<()> {
    if ptr.is_null() {
        bail!("Cannot unmap null pointer");
    }

    if len == 0 {
        bail!("Cannot unmap with length 0");
    }

    let ret = unsafe { unix::munmap(ptr as *mut c_void, len) };

    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("munmap(2) failed");
    }

    Ok(())
}

/// Safe wrapper for madvise(2)
pub fn safe_madvise(
    ptr: *mut u8,
    len: usize,
    advice: i32,
) -> Result<()> {
    if ptr.is_null() {
        bail!("Cannot madvise null pointer");
    }

    if len == 0 {
        bail!("Cannot madvise with length 0");
    }

    let ret =
        unsafe { unix::madvise(ptr as *mut c_void, len, advice) };

    if ret != 0 {
        return Err(std::io::Error::last_os_error()).context(
            format!("madvise(2) failed with advice={}", advice),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests;
