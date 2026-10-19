//! Integration tests for zero-fill verification and file truncation behavior.

use mmap_alloc::AllocMmap;
use std::fs;

/// Tests that newly created files are completely zero-filled.
#[test]
fn test_file_is_zero_filled_on_creation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("zero_filled.bin");

    AllocMmap::create(&file_path, 8192).unwrap();

    let data = fs::read(&file_path).unwrap();
    assert!(data.iter().all(|&b| b == 0), "File should be zero-filled");

    fs::remove_file(&file_path).ok();
}

/// Tests that recreating a file with different size truncates and zero-fills it.
#[test]
fn test_truncate_clears_previous_content() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("truncate_test.bin");

    // First write
    {
        let alloc_mmap = AllocMmap::create(&file_path, 512).unwrap();
        unsafe {
            let slice = std::slice::from_raw_parts_mut(alloc_mmap.as_slice().as_ptr() as *mut u8, 512);
            slice.fill(255);
        }
        alloc_mmap.flush().unwrap();
    }

    // Second write (should truncate)
    let alloc_mmap = AllocMmap::create(&file_path, 256).unwrap();
    alloc_mmap.flush().unwrap();

    let data = fs::read(&file_path).unwrap();
    assert_eq!(data.len(), 256, "File should be truncated to new size");
    assert!(data.iter().all(|&b| b == 0), "New file should be zero-filled");

    fs::remove_file(&file_path).ok();
}
