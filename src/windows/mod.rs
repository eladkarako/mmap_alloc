//! Windows-specific file operations for memory-mapped I/O and file allocation.
//!
//! This module provides platform-specific utilities for Windows to:
//! - Allocate file space efficiently using Windows-specific file handling
//! - Create and manage memory-mapped files for high-performance file access

use crate::ffi_windows;
use crate::mmap::MmapMut;
use anyhow::Result;
use std::fs::File;

/// Allocates file space on disk using Windows file handling mechanisms.
///
/// Allocates the specified amount of space for a file on Windows by calling
/// Windows-specific file information APIs. This is useful for pre-allocating
/// disk space and avoiding fragmentation or out-of-space errors during subsequent writes.
///
/// # Arguments
///
/// * `file` - A reference to an open [`File`] handle to allocate space for
/// * `size` - The number of bytes to allocate for the file
///
/// # Returns
///
/// Returns `Ok(())` on successful allocation, or an [`anyhow::Error`] if the operation fails.
///
/// # Errors
///
/// Returns an error if:
/// - The file handle is invalid
/// - Insufficient disk space is available
/// - The underlying Windows API call fails
///
/// # Example
///
/// ```ignore
/// use std::fs::File;
/// use std::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .write(true)
///     .create(true)
///     .open("myfile.bin")?;
///
/// allocate\_file(&file, 1024 * 1024)?; // Allocate 1 MB
/// ```
pub fn allocate_file(
    file: &File,
    size: u64,
) -> Result<()> {
    ffi_windows::safe_set_file_information_by_handle(file, size)
}

/// Creates a memory-mapped view of a file for efficient random access.
///
/// Maps a file into memory using Windows memory-mapping APIs, allowing the file
/// contents to be accessed as if they were in RAM. This enables high-performance
/// random access and reduces the number of system calls needed for file I/O.
///
/// # Arguments
///
/// * `file` - A reference to an open [`File`] to map into memory
/// * `size` - The size in bytes of the region to map
///
/// # Returns
///
/// Returns a [`MmapMut`] instance on success, allowing mutable access to the mapped
/// memory region, or an [`anyhow::Error`] if mapping fails.
///
/// # Errors
///
/// Returns an error if:
/// - The file handle is invalid
/// - The requested size exceeds the file size
/// - Memory mapping is not supported or fails
/// - Insufficient virtual address space is available
///
/// # Safety
///
/// The returned [`MmapMut`] maintains a reference to the underlying file.
/// The file must not be truncated or closed while the memory map is in use.
///
/// # Example
///
/// ```ignore
/// use std::fs::File;
/// use std::fs::OpenOptions;
///
/// let file = OpenOptions::new()
///     .read(true)
///     .write(true)
///     .open("myfile.bin")?;
///
/// let mmap = mmap\_file(&file, 4096)?;
/// // Access file contents through the mmap
/// ```
pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}

#[cfg(test)]
mod tests;
