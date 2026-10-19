use crate::AllocMmap;
use std::fs;

#[test]
fn test_alloc_mmap_create() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 1024).unwrap();
    assert_eq!(alloc_mmap.size(), 1024);
    assert!(file_path.exists());

    fs::remove_file(&file_path).ok();
}

#[test]
fn test_alloc_mmap_rejects_zero_size() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.bin");

    let result = AllocMmap::create(&file_path, 0);
    assert!(result.is_err());
}

#[test]
fn test_alloc_mmap_as_slice() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 512).unwrap();
    let slice = alloc_mmap.as_slice();
    assert_eq!(slice.len(), 512);
    // Should be zero-filled
    assert!(slice.iter().all(|&b| b == 0));

    fs::remove_file(&file_path).ok();
}
