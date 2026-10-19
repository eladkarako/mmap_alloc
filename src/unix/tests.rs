#[cfg(unix)]

use crate::unix::*;
use std::fs::OpenOptions;

#[test]
fn test_allocate_and_map_file() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(temp_file.path())
        .unwrap();

    allocate_file(&file, 2048).unwrap();
    let mmap = mmap_file(&file, 2048).unwrap();
    assert_eq!(mmap.len(), 2048);
}
