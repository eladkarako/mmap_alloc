//! Fast memory-mapped file allocation with optional parallel writes.
//!
//! This crate provides cross-platform (Unix/Windows) utilities for:
//! - Pre-allocating files on disk
//! - Memory-mapping files for fast I/O
//! - Writing to mapped segments in parallel
//!
//! # Features
//! - `multithreaded`: Enable parallel write support with `write_parallel()`
//!
//! # Example
//! ```ignore
//! use mmap\_alloc::AllocMmap;
//!
//! let alloc = AllocMmap::create("data.bin", 1024 * 1024)?;
//! alloc.write\_parallel(
//!     |seg1| { seg1.fill(b'A'); Ok(()) },
//!     |seg2| { seg2.fill(b'B'); Ok(()) },
//! )?;
//! alloc.flush()?;
//! ```

pub mod bindings;
pub mod mmap;

#[cfg(unix)]
mod ffi_unix;
#[cfg(unix)]
/// Unix-specific memory mapping operations.
pub mod unix;

#[cfg(windows)]
mod ffi_windows;
#[cfg(windows)]
/// Windows-specific memory mapping operations.
pub mod windows;

use anyhow::{bail, Context, Result};
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::Arc;

#[cfg(unix)]
use crate::unix as os;
#[cfg(windows)]
use crate::windows as os;

pub use mmap::MmapMut;

/// A memory-mapped file allocator with optional parallel write support.
///
/// Provides fast pre-allocation and memory-mapped access to files on both Unix and Windows.
/// When the `multithreaded` feature is enabled, supports splitting the buffer into segments
/// for parallel writes.
pub struct AllocMmap {
    mmap: Arc<MmapMut>,
    size: usize,
}

impl AllocMmap {
    /// Creates a new memory-mapped file at the given path with pre-allocation.
    ///
    /// # Arguments
    /// * `path` - File path (will be created/truncated)
    /// * `size` - File size in bytes (must be > 0)
    ///
    /// # Errors
    /// Returns error if file creation, allocation, or mmap fails.
    pub fn create<P: AsRef<Path>>(path: P, size: u64) -> Result<Self> {
        if size == 0 {
            bail!("File size must be greater than 0");
        }

        let path = path.as_ref();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .context(format!("Failed to create file at {:?}", path))?;

        os::allocate_file(&file, size).context("Failed to pre-allocate file space")?;
        let mut mmap = os::mmap_file(&file, size as usize)
            .context("Failed to memory map file")?;

        // Initialize to zero for safety (before wrapping in Arc)
        mmap.as_mut_slice().fill(0);

        Ok(AllocMmap {
            mmap: Arc::new(mmap),
            size: size as usize,
        })
    }

    /// Returns a read-only slice of the mapped memory.
    pub fn as_slice(&self) -> &[u8] {
        &*self.mmap
    }

    /// Returns the total size of the allocated mmap in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Flushes all changes to disk synchronously (blocks until complete).
    pub fn flush(&self) -> Result<()> {
        #[cfg(unix)]
        self.mmap.msync_sync()?;

        #[cfg(windows)]
        self.mmap.flush_sync()?;

        Ok(())
    }

    /// Flushes changes asynchronously (may return before write completes).
    pub fn flush_async(&self) -> Result<()> {
        #[cfg(unix)]
        self.mmap.msync_async()?;

        #[cfg(windows)]
        self.mmap.flush_async_internal()?;

        Ok(())
    }

    /// Writes to the buffer in parallel using two threads, each handling a segment.
    ///
    /// Splits the buffer at the midpoint and spawns two threads:
    /// - `f1` writes to the first half
    /// - `f2` writes to the second half
    ///
    /// # Errors
    /// - Returns error if file size < 2 bytes
    /// - Returns error if either closure panics or fails
    ///
    /// # Example
    /// ```ignore
    /// alloc\_mmap.write\_parallel(
    ///     |segment| { segment.fill(b'A'); Ok(()) },
    ///     |segment| { segment.fill(b'B'); Ok(()) },
    /// )?;
    /// ```
    #[cfg(feature = "multithreaded")]
    pub fn write_parallel<F1, F2>(&self, f1: F1, f2: F2) -> Result<()>
    where
        F1: FnOnce(&mut [u8]) -> Result<()> + Send + 'static,
        F2: FnOnce(&mut [u8]) -> Result<()> + Send + 'static,
    {
        use std::thread;

        const MIN_SIZE: usize = 2;
        if self.size < MIN_SIZE {
            bail!(
                "File size too small for parallel writes (minimum {} bytes)",
                MIN_SIZE
            );
        }

        let mid = self.size / 2;

        // Clone Arc for thread safety
        let mmap_clone1 = Arc::clone(&self.mmap);
        let mmap_clone2 = Arc::clone(&self.mmap);

        // Spawn first thread
        let handle1 = thread::spawn(move || {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(mmap_clone1.as_ptr(), mid)
            };
            f1(slice)
        });

        // Spawn second thread
        let handle2 = thread::spawn(move || {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(mmap_clone2.as_ptr().add(mid), self.size - mid)
            };
            f2(slice)
        });

        // Wait for completion and collect errors
        match (handle1.join(), handle2.join()) {
            (Ok(result1), Ok(result2)) => {
                result1.context("Thread 1 write failed")?;
                result2.context("Thread 2 write failed")?;
                Ok(())
            }
            (Err(_), _) => bail!("Thread 1 panicked during write_parallel"),
            (_, Err(_)) => bail!("Thread 2 panicked during write_parallel"),
        }
    }
}

#[cfg(test)]
mod tests;
