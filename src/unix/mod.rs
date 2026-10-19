use crate::mmap::MmapMut;
use anyhow::{bail, Context};
use std::fs::File;
use std::os::unix::io::AsRawFd;

pub fn allocate_file(
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
        unsafe { libc::fallocate(fd, 0, 0, size as libc::off_t) };
    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("fallocate(2) failed");
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
