use crate::mmap::MmapMut;
use std::fs::OpenOptions;

#[test]
fn test_mmap_mut_from_file() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(temp_file.path())
        .unwrap();

    // Pre-allocate on Unix-like systems
    #[cfg(unix)]
    {
        crate::ffi_unix::safe_fallocate(&file, 4096).unwrap();
    }
    #[cfg(windows)]
    {
        crate::ffi_windows::safe_set_file_information_by_handle(&file, 4096).unwrap();
    }

    let mmap = MmapMut::map_mut(&file, 4096).unwrap();
    assert_eq!(mmap.len(), 4096);
    assert!(!mmap.ptr.is_null());
}

#[test]
fn test_mmap_mut_is_empty() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(temp_file.path())
        .unwrap();

    #[cfg(unix)]
    crate::ffi_unix::safe_fallocate(&file, 1024).unwrap();
    #[cfg(windows)]
    crate::ffi_windows::safe_set_file_information_by_handle(&file, 1024).unwrap();

    let mmap = MmapMut::map_mut(&file, 1024).unwrap();
    assert!(!mmap.is_empty());
}

