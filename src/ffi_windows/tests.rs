#[cfg(windows)]

use crate::ffi_windows::*;
use crate::bindings::windows;
use std::fs::OpenOptions;

#[test]
fn test_safe_create_file_mapping_rejects_zero_size() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(temp_dir.path().join("test.bin"))
        .unwrap();

    let result = safe_create_file_mapping(&file, 0);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("size 0"));
}

#[test]
fn test_safe_map_view_rejects_zero_length() {
    let invalid_handle = windows::INVALID_HANDLE_VALUE;
    let result = safe_map_view_of_file_ex(invalid_handle, 0);
    assert!(result.is_err());
}

#[test]
fn test_safe_set_file_information_rejects_oversized() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(temp_dir.path().join("test2.bin"))
        .unwrap();

    let result = safe_set_file_information_by_handle(&file, (i64::MAX as u64) + 1);
    assert!(result.is_err());
}

#[test]
fn test_safe_unmap_null_pointer() {
    let result = safe_unmap_view_of_file(std::ptr::null_mut());
    assert!(result.is_err());
}
