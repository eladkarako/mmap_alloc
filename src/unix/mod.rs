use crate::mmap::MmapMut;
#[allow(unused_imports)]
use anyhow::{bail, Context, Result};
use std::{fs::File, os::unix::io::AsRawFd};

pub fn allocate_file(
    file: &File,
    size: u64,
) -> Result<()> {
    validate_allocation_request(file, size)?;
    perform_fallocate(file, size)
}

pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}

fn validate_allocation_request(
    file: &File,
    size: u64,
) -> Result<()> {
    validate_file_descriptor(file)?;
    validate_allocation_size_is_nonzero(size)?;
    validate_allocation_size_within_limits(size)?;
    Ok(())
}

fn validate_file_descriptor(file: &File) -> Result<()> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        bail!("Invalid file descriptor: {}", fd);
    }
    Ok(())
}

fn validate_allocation_size_is_nonzero(size: u64) -> Result<()> {
    if size == 0 {
        bail!("Cannot allocate file with size 0");
    }
    Ok(())
}

fn validate_allocation_size_within_limits(size: u64) -> Result<()> {
    if size > i64::MAX as u64 {
        bail!(
            "Requested allocation size {} exceeds maximum i64",
            size
        );
    }
    Ok(())
}

fn perform_fallocate(
    file: &File,
    size: u64,
) -> Result<()> {
    let fd = file.as_raw_fd();
    let ret =
        unsafe { libc::fallocate(fd, 0, 0, size as libc::off_t) };

    if ret != 0 {
        return Err(std::io::Error::last_os_error())
            .context("fallocate(2) failed");
    }

    Ok(())
}

#[cfg(test)]
mod tests;
