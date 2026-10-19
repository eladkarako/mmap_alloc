#[cfg(unix)]

use crate::ffi_unix::*;
use std::fs::OpenOptions;

#[test]
fn test_safe_fallocate_rejects_zero_size() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(temp_dir.path().join("test.bin"))
        .unwrap();

    let result = safe_fallocate(&file, 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("size 0"));
}

#[test]
fn test_safe_fallocate_rejects_oversized() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(temp_dir.path().join("test2.bin"))
        .unwrap();

    let result = safe_fallocate(&file, (i64::MAX as u64) + 1);
    assert!(result.is_err());
}

#[test]
fn test_safe_munmap_rejects_null() {
    let result = safe_munmap(std::ptr::null_mut(), 1024);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("null"));
}

#[test]
fn test_safe_munmap_rejects_zero_length() {
    let result = safe_munmap(1024 as *mut u8, 0);
    assert!(result.is_err());
}
