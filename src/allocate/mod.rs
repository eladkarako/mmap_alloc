use anyhow::Result;
use std::fs::File;

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
use unix as os_impl;
#[cfg(windows)]
use windows as os_impl;

/// Allocate disk space for a file before mapping
pub fn allocate_file(
    file: &File,
    size: u64,
) -> Result<()> {
    os_impl::allocate_file_impl(file, size)
}
