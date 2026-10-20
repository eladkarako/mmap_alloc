use anyhow::{bail, Context};
use std::fs::File;
use std::os::unix::io::AsRawFd;

pub fn allocate_file_impl(
    file: &File,
    size: u64,
) -> Result<()> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        bail!("Invalid file descriptor")
    }
    if size == 0 {
        bail!("Cannot allocate size 0")
    }
    if size > i64::MAX as u64 {
        bail!("Size exceeds i64::MAX")
    }

    let ret =
        unsafe { libc::fallocate(fd, 0, 0, size as libc::off_t) };
    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("fallocate failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests;
