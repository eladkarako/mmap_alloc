//! Integration tests for edge cases: single-byte files, large files, and page alignment.

use mmap_alloc::AllocMmap;
use std::fs;

/// Tests creating and writing to a single-byte file.
#[test]
fn test_single_byte_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("single_byte.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 1).unwrap();
    assert_eq!(alloc_mmap.size(), 1);

    unsafe {
        let slice = std::slice::from_raw_parts_mut(alloc_mmap.as_slice().as_ptr() as *mut u8, 1);
        slice[0] = 255;
    }

    alloc_mmap.flush().unwrap();

    let data = fs::read(&file_path).unwrap();
    assert_eq!(data[0], 255);

    fs::remove_file(&file_path).ok();
}

/// Tests creating and managing a 1 MB file.
#[test]
fn test_large_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("large.bin");
    let size: usize = 1024 * 1024; // 1 MiB

    let alloc_mmap = AllocMmap::create(&file_path, size as u64).unwrap();
    assert_eq!(alloc_mmap.size(), size);

    alloc_mmap.flush().unwrap();

    let metadata = fs::metadata(&file_path).unwrap();
    assert_eq!(metadata.len() as usize, size);

    fs::remove_file(&file_path).ok();
}

/// Tests creating a file with a page-aligned size (4096 bytes).
#[test]
fn test_page_aligned_size() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("page_aligned.bin");
    let size: usize = 4096; // Common page size

    let alloc_mmap = AllocMmap::create(&file_path, size as u64).unwrap();
    assert_eq!(alloc_mmap.size(), 4096);

    fs::remove_file(&file_path).ok();
}
