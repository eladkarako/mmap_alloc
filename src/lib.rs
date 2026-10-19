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
use std::{
    fs::{File, OpenOptions},
    path::Path,
    sync::Arc,
};

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
        validate_file_size_for_parallel_writes(self.size)?;
        let (handle1, handle2) = spawn_parallel_write_threads(
            &self.mmap, self.size, f1, f2,
        )?;
        wait_for_parallel_write_completion(handle1, handle2)
    }
}

fn validate_allocation_size(size: u64) -> Result<()> {
    if size == 0 {
        bail!("File size must be greater than 0");
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
    slice.fill(0);
}

#[cfg(feature = "multithreaded")]
fn validate_file_size_for_parallel_writes(
    size: usize
) -> Result<()> {
    const MINIMUM_PARALLEL_WRITE_SIZE: usize = 2;
    if size < MINIMUM_PARALLEL_WRITE_SIZE {
        bail!(
            "File size too small for parallel writes (minimum {} bytes)",
            MINIMUM_PARALLEL_WRITE_SIZE
        );
    }
    Ok(())
}

#[cfg(feature = "multithreaded")]
fn spawn_parallel_write_threads<F1, F2>(
    mmap: &Arc<MmapMut>,
    total_size: usize,
    f1: F1,
    f2: F2,
) -> Result<(
    std::thread::JoinHandle<Result<()>>,
    std::thread::JoinHandle<Result<()>>,
)>
where
    F1: FnOnce(&mut [u8]) -> Result<()> + Send + 'static,
    F2: FnOnce(&mut [u8]) -> Result<()> + Send + 'static,
{
    use std::thread;

    let midpoint = calculate_split_midpoint(total_size);
    let mmap_clone1 = Arc::clone(mmap);
    let mmap_clone2 = Arc::clone(mmap);

    let handle1 = thread::spawn(move || {
        let slice =
            create_mutable_slice_from_start(&mmap_clone1, midpoint)?;
        f1(slice)
    });

    let handle2 = thread::spawn(move || {
        let slice = create_mutable_slice_from_offset(
            &mmap_clone2,
            midpoint,
            total_size,
        )?;
        f2(slice)
    });

    Ok((handle1, handle2))
}

#[cfg(feature = "multithreaded")]
fn calculate_split_midpoint(total_size: usize) -> usize {
    total_size / 2
}

#[cfg(feature = "multithreaded")]
fn create_mutable_slice_from_start(
    mmap: &Arc<MmapMut>,
    length: usize,
) -> Result<&mut [u8]> {
    let slice = unsafe {
        std::slice::from_raw_parts_mut(mmap.as_ptr(), length)
    };
    Ok(slice)
}

#[cfg(feature = "multithreaded")]
fn create_mutable_slice_from_offset(
    mmap: &Arc<MmapMut>,
    offset: usize,
    remaining_length: usize,
) -> Result<&mut [u8]> {
    let slice = unsafe {
        std::slice::from_raw_parts_mut(
            mmap.as_ptr().add(offset),
            remaining_length - offset,
        )
    };
    Ok(slice)
}

#[cfg(feature = "multithreaded")]
fn wait_for_parallel_write_completion(
    handle1: std::thread::JoinHandle<Result<()>>,
    handle2: std::thread::JoinHandle<Result<()>>,
) -> Result<()> {
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
