// C:\Users\Elad\RustroverProjects\mmap_alloc\src\unix\mod.rs
#[cfg(unix)]
use crate::mmap::MmapMut;
use anyhow::{bail, Context};
use std::{fs::File, os::unix::io::AsRawFd};
pub fn allocate_file(
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
pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}
