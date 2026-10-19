//! Integration tests for synchronous and asynchronous flush operations.

use mmap_alloc::AllocMmap;
use std::fs;

/// Tests synchronous flush persists data correctly to disk.
#[test]
fn test_flush_sync() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("flush_sync.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 2048).unwrap();

    // Write pattern
    unsafe {
        let slice = std::slice::from_raw_parts_mut(alloc_mmap.as_slice().as_ptr() as *mut u8, 2048);
        for (i, byte) in slice.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }
    }

    // Sync flush
    alloc_mmap.flush().unwrap();

    // Verify on disk
    let data = fs::read(&file_path).unwrap();
    for (i, byte) in data.iter().enumerate() {
        assert_eq!(*byte, (i % 256) as u8);
    }

    fs::remove_file(&file_path).ok();
}

/// Tests asynchronous flush completes without errors.
#[test]
fn test_flush_async() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("flush_async.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 1024).unwrap();

    unsafe {
        let slice = std::slice::from_raw_parts_mut(alloc_mmap.as_slice().as_ptr() as *mut u8, 1024);
        slice.fill(42);
    }

    // Async flush (doesn't guarantee immediate write, but shouldn't error)
    alloc_mmap.flush_async().unwrap();

    fs::remove_file(&file_path).ok();
}
