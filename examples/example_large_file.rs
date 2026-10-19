//! Large file write example demonstrating efficiency with larger allocations.
//!
//! This example creates a larger memory-mapped file and writes structured data
//! to demonstrate the crate's efficiency with substantial file sizes.

use mmap_alloc::AllocMmap;

fn main() -> anyhow::Result<()> {
    let file_path = "output_large.bin";
    let file_size: u64 = 10 * 1024 * 1024; // 10 MB

    println!("Creating a {} MB memory-mapped file...", file_size / (1024 * 1024));
    let mmap = AllocMmap::create(file_path, file_size)?;

    println!("✓ File created and pre-allocated");
    println!("  Path: {}", file_path);
    println!("  Size: {} MB", mmap.size() / (1024 * 1024));

    // Write pattern in chunks (demonstrating efficiency)
    let slice = mmap.as_slice();
    let chunk_size = 4096;

    println!("\nWriting pattern data...");
    for (chunk_idx, window) in slice.chunks(chunk_size).enumerate() {
        if chunk_idx % 256 == 0 {
            println!("  Processed {} KB...", (chunk_idx * chunk_size) / 1024);
        }

        // Write pattern: cycle through bytes
        unsafe {
            let mutable_ptr = window.as_ptr() as *mut u8;
            for (i, byte) in (*mutable_ptr as *mut [u8])[..window.len()].iter_mut().enumerate() {
                *byte = ((chunk_idx + i) % 256) as u8;
            }
        }
    }

    println!("✓ Pattern write completed");

    // Async flush for non-blocking write
    mmap.flush_async()?;
    println!("✓ Asynchronous flush initiated");

    // Verify file was created
    let file_size_on_disk = std::fs::metadata(file_path)?.len();
    println!("\n✓ File size on disk: {} MB", file_size_on_disk / (1024 * 1024));

    std::fs::remove_file(file_path).ok();
    Ok(())
}
