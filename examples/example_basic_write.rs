//! Basic example of creating and writing to a memory-mapped file.
//!
//! This example demonstrates the simplest use case: creating a file,
//! writing some data to it, and flushing to disk.

use mmap_alloc::AllocMmap;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let file_path = "output_basic.bin";
    let file_size = 1024; // 1 KB

    // Create a memory-mapped file with pre-allocated space
    let mmap = AllocMmap::create(file_path, file_size)?;

    println!("✓ Created memory-mapped file: {}", file_path);
    println!("  File size: {} bytes", mmap.size());

    // Get mutable access to write data
    let slice = mmap.as_slice();
    println!("  Current content (first 32 bytes): {:?}", &slice[..32.min(slice.len())]);

    // Write a message by casting to mutable slice
    // Note: In real code, you'd use interior mutability or restructure for mutable access
    let message = b"Hello from mmap_alloc!";
    unsafe {
        let mutable_slice = slice.as_ptr() as *mut [u8];
        (*mutable_slice)[..message.len()].copy_from_slice(message);
    }

    // Flush changes to disk synchronously
    mmap.flush()?;
    println!("✓ Data flushed to disk synchronously");

    println!("\n✓ Example completed successfully!");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
