//! Parallel write example using the `multithreaded` feature.
//!
//! This example demonstrates how to split a memory-mapped file into
//! two halves and write to them concurrently from separate threads.
//!
//! # Note
//! This example requires the `multithreaded` feature to be enabled:
//! ```sh
//! cargo run --example parallel\_write --features multithreaded
//! ```

#[cfg(feature = "multithreaded")]
use mmap_alloc::AllocMmap;
#[cfg(feature = "multithreaded")]
use std::sync::Arc;

#[cfg(feature = "multithreaded")]
fn main() -> anyhow::Result<()> {
    let file_path = "output_parallel.bin";
    let file_size = 4096; // 4 KB

    // Create and allocate the memory-mapped file
    let mmap = AllocMmap::create(file_path, file_size)?;
    println!("✓ Created memory-mapped file: {}", file_path);
    println!("  File size: {} bytes", mmap.size());

    // Perform parallel writes: fill first half with 'A' and second half with 'B'
    mmap.write_parallel(
        |first_half| {
            println!("  [Thread 1] Writing to first half ({} bytes)...", first_half.len());
            first_half.fill(b'A');
            Ok(())
        },
        |second_half| {
            println!("  [Thread 2] Writing to second half ({} bytes)...", second_half.len());
            second_half.fill(b'B');
            Ok(())
        },
    )?;

    println!("✓ Parallel writes completed");

    // Flush to disk
    mmap.flush()?;
    println!("✓ Data flushed to disk");

    // Verify the result
    let data = std::fs::read(file_path)?;
    let midpoint = data.len() / 2;
    let first_ok = data[..midpoint].iter().all(|&b| b == b'A');
    let second_ok = data[midpoint..].iter().all(|&b| b == b'B');

    if first_ok && second_ok {
        println!("✓ Verification passed: file contains expected pattern");
    } else {
        println!("✗ Verification failed: unexpected data pattern");
    }

    std::fs::remove_file(file_path).ok();
    Ok(())
}

#[cfg(not(feature = "multithreaded"))]
fn main() {
    eprintln!("This example requires the 'multithreaded' feature.");
    eprintln!("Run with: cargo run --example parallel_write --features multithreaded");
    std::process::exit(1);
}
