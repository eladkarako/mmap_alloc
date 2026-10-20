use super::*;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_alloc_mmap_create() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let size = 1024u64;

    let mmap = AllocMmap::create(path, size)?;
    assert_eq!(mmap.size(), 1024);
    Ok(())
}

#[test]
fn test_alloc_mmap_write_bytes() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let data = b"Hello, World!";
    let size = 1024u64;

    let mmap = AllocMmap::create(path, size)?;
    mmap.write_bytes(data)?;

    let slice = mmap.as_slice();
    assert_eq!(&slice[..data.len()], data);
    Ok(())
}
#[test]
fn test_alloc_mmap_builder() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let size = 2048u64;

    let mmap =
        AllocMmap::builder(path, size).with_partitions(4).build()?;

    assert_eq!(mmap.size(), 2048);
    Ok(())
}

#[test]
fn test_alloc_mmap_builder_write_bytes() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let data = b"Test data";
    let size = 512u64;

    let mmap = AllocMmap::builder(path, size)
        .build_and_write_bytes(data)?;

    let slice = mmap.as_slice();
    assert_eq!(&slice[..data.len()], data);
    Ok(())
}

#[test]
fn test_write_parallel() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let size = 10000u64;

    let mmap = AllocMmap::create(path, size)?;
    mmap.write_parallel(|partition_idx, chunk| {
        // Fill each partition with its index as a byte value
        chunk.fill(partition_idx as u8);
        Ok(())
    })?;

    mmap.flush()?;
    Ok(())
}

#[test]
fn test_write_parallel_with_partitions() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let size = 10000u64;

    let mmap = AllocMmap::create(path, size)?;
    mmap.write_parallel_with_partitions(
        Some(8),
        |partition_idx, chunk| {
            chunk.fill(partition_idx as u8);
            Ok(())
        },
    )?;

    mmap.flush()?;
    Ok(())
}

#[test]
fn test_flush() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let data = b"Data to flush";
    let size = 1024u64;

    let mmap = AllocMmap::create(path, size)?;
    mmap.write_bytes(data)?;
    mmap.flush()?;

    // Verify data persists
    let file_data = fs::read(path)?;
    assert_eq!(&file_data[..data.len()], data);
    Ok(())
}

#[test]
fn test_zero_initialization() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    let size = 512u64;

    let mmap = AllocMmap::create(path, size)?;
    let slice = mmap.as_slice();

    // Verify memory is zero-initialized
    assert!(slice.iter().all(|&b| b == 0));
    Ok(())
}
