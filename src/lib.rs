pub mod mmap;
#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix as os;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows as os;

use anyhow::{bail, Context, Result};
pub use mmap::MmapMut;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::Arc;

pub struct AllocMmap {
    mmap: Arc<MmapMut>,
    size: usize,
}
impl AllocMmap {
    pub fn create<P: AsRef<Path>>(
        path: P,
        size: u64,
    ) -> Result<Self> {
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
            .context(format!(
                "Failed to create file at {:?}",
                path
            ))?;
        os::allocate_file(&file, size)
            .context("Failed to pre-allocate file space")?;
        let mut mmap = os::mmap_file(&file, size as usize)
            .context("Failed to memory map file")?;
        mmap.as_mut_slice().fill(0);
        Ok(AllocMmap { mmap: Arc::new(mmap), size: size as usize })
    }
    pub fn as_slice(&self) -> &[u8] {
        &*self.mmap
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn flush(&self) -> Result<()> {
        self.mmap.flush_sync()
    }
    pub fn flush_async(&self) -> Result<()> {
        self.mmap.flush_async()
    }
    #[cfg(feature = "multithreaded")]
    pub fn write_parallel<F1, F2>(
        &self,
        f1: F1,
        f2: F2,
    ) -> Result<()>
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
        let mmap_clone1 = Arc::clone(&self.mmap);
        let mmap_clone2 = Arc::clone(&self.mmap);
        let handle1 = thread::spawn(move || {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    mmap_clone1.as_ptr(),
                    mid,
                )
            };
            f1(slice)
        });
        let handle2 = thread::spawn(move || {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    mmap_clone2.as_ptr().add(mid),
                    self.size - mid,
                )
            };
            f2(slice)
        });
        match (handle1.join(), handle2.join()) {
            (Ok(result1), Ok(result2)) => {
                result1.context("Thread 1 write failed")?;
                result2.context("Thread 2 write failed")?;
                Ok(())
            }
            (Err(_), _) => {
                bail!("Thread 1 panicked during write_parallel")
            }
            (_, Err(_)) => {
                bail!("Thread 2 panicked during write_parallel")
            }
        }
    }
}
