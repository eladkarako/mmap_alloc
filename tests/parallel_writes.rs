//! Integration tests for parallel write operations.

#[cfg(feature = "multithreaded")]
use mmap_alloc::AllocMmap;

/// Tests writing to two halves of a file in parallel threads with correct data separation.
#[cfg(feature = "multithreaded")]
#[test]
fn test_write_parallel_two_halves() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("parallel.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 4096).unwrap();

    alloc_mmap
        .write_parallel(
            |slice| {
                for (i, byte) in slice.iter_mut().enumerate() {
                    *byte = (i % 128) as u8;
                }
                Ok(())
            },
            |slice| {
                for (i, byte) in slice.iter_mut().enumerate() {
                    *byte = ((i + 128) % 256) as u8;
                }
                Ok(())
            },
        )
        .unwrap();

    alloc_mmap.flush().unwrap();

    let data = std::fs::read(&file_path).unwrap();
    let mid = data.len() / 2;

    // Verify first half
    for i in 0..mid {
        assert_eq!(data[i], (i % 128) as u8);
    }

    // Verify second half
    for i in mid..data.len() {
        assert_eq!(data[i], ((i % 256 + 128) % 256) as u8);
    }

    std::fs::remove_file(&file_path).ok();
}

/// Tests that parallel writes fail gracefully on files smaller than minimum size.
#[cfg(feature = "multithreaded")]
#[test]
fn test_write_parallel_too_small_fails() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("parallel_small.bin");

    let alloc_mmap = AllocMmap::create(&file_path, 1).unwrap();

    let result = alloc_mmap.write_parallel(
        |_| Ok(()),
        |_| Ok(()),
    );

    assert!(result.is_err());
    std::fs::remove_file(&file_path).ok();
}
