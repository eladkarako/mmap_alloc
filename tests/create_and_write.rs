//! Integration tests for creating, writing, and reading mmap files.

use mmap_alloc::AllocMmap;
use std::fs;

/// Tests creating a file, writing data to it via mmap, flushing, and verifying the data persists.
#[test]
fn test_create_write_and_read() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("integration_write.bin");

    // Create a 4 KiB file
    let alloc_mmap = AllocMmap::create(&file_path, 4096).unwrap();

    // Write test data
    {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(alloc_mmap.as_slice().as_ptr() as *mut u8, 4096)
        };
        slice[0..5].copy_from_slice(b"hello");
        slice[100..105].copy_from_slice(b"world");
    }

    alloc_mmap.flush().unwrap();

    // Read back and verify
    let data = fs::read(&file_path).unwrap();
    assert_eq!(&data[0..5], b"hello");
    assert_eq!(&data[100..105], b"world");

    fs::remove_file(&file_path).ok();
}

/// Tests that the created file on disk matches the requested size exactly.
#[test]
fn test_file_size_verification() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("size_test.bin");
    let target_size: usize = 65536;

    AllocMmap::create(&file_path, target_size as u64).unwrap();

    let metadata = fs::metadata(&file_path).unwrap();
    assert_eq!(metadata.len() as usize, target_size);

    fs::remove_file(&file_path).ok();
}
