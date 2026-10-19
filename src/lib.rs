// C:\Users\Elad\RustroverProjects\mmap_alloc\src\lib.rs
pub mod mmap;
#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as os;
#[cfg(windows)]
mod windows;
use anyhow::{bail, Context, Result};
pub use mmap::MmapMut;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;
#[cfg(windows)]
use windows as os;
pub struct AllocMmap {
    mmap: Arc<MmapMut>,
    size: usize,
}
impl AllocMmap {
    pub fn create<P: AsRef<Path>>(
        path: P,
        size: u64,
    ) -> Result<Self> {
        validate_allocation_size(size)?;
        let path = path.as_ref();
        let file = create_file_at_path(path)?;
        os::allocate_file(&file, size)
            .context("Failed to pre-allocate file space")?;
        let mmap = initialize_and_map_file(&file, size)?;
        Ok(AllocMmap { mmap: Arc::new(mmap), size: size as usize })
    }
    pub fn as_slice(&self) -> &[u8] {
        &*self.mmap
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn flush(&self) -> Result<()> {
        self.mmap.flush()
    }
    pub fn write_parallel<F>(
        &self,
        partition_fn: F,
    ) -> Result<()>
    where
        F: Fn(usize, &mut [u8]) -> Result<()> + Send + Sync,
    {
        self.write_parallel_with_partitions(None, partition_fn)
    }
    pub fn write_parallel_with_partitions<F>(
        &self,
        partition_count: Option<usize>,
        partition_fn: F,
    ) -> Result<()>
    where
        F: Fn(usize, &mut [u8]) -> Result<()> + Send + Sync,
    {
        let thread_count = partition_count.unwrap_or_else(|| {
            (num_cpus::get()).saturating_sub(1).max(1)
        });
        let chunk_size =
            (self.size + thread_count - 1) / thread_count;
        let partition_fn = Arc::new(partition_fn);
        std::thread::scope(|s| {
            let results: Vec<_> = (0..thread_count)
                .map(|i| {
                    let start = i * chunk_size;
                    let end = (start + chunk_size).min(self.size);
                    let partition_fn = Arc::clone(&partition_fn);
                    s.spawn(move || {
                        let slice = unsafe {
                            std::slice::from_raw_parts_mut(
                                self.mmap.as_ptr().add(start),
                                end - start,
                            )
                        };
                        partition_fn(i, slice)
                    })
                })
                .collect();
            for (i, result) in results.into_iter().enumerate() {
                result
                    .join()
                    .map_err(|_| {
                        anyhow::anyhow!("Thread {} panicked", i)
                    })?
                    .context(format!("Thread {} write failed", i))?;
            }
            Ok(())
        })
    }
}
fn validate_allocation_size(size: u64) -> Result<()> {
    if size == 0 {
        bail!("File size must be greater than 0")
    }
    Ok(())
}
fn create_file_at_path<P: AsRef<Path>>(path: P) -> Result<File> {
    let path = path.as_ref();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .context(format!("Failed to create file at {:?}", path))
}
fn initialize_and_map_file(
    file: &File,
    size: u64,
) -> Result<MmapMut> {
    let mut mmap = os::mmap_file(file, size as usize)
        .context("Failed to memory map file")?;
    let slice = mmap.as_mut_slice();
    zero_initialize_slice(slice);
    Ok(mmap)
}
fn zero_initialize_slice(slice: &mut [u8]) {
    slice.fill(0)
}
