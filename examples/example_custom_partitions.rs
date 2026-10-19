// C:\Users\Elad\RustroverProjects\mmap_alloc\examples\example_custom_partitions.rs
use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_custom.bin";
    let file_size: u64 = 8 * 1024 * 1024;
    let custom_partitions = 4;
    println!(
        "Creating {} MB file with {} explicit partitions...",
        file_size / (1024 * 1024),
        custom_partitions
    );
    let mmap = AllocMmap::create(file_path, file_size)?;
    println!("✓ File created and pre-allocated");
    println!("  Path: {}", file_path);
    println!("  Size: {} MB", mmap.size() / (1024 * 1024));
    println!("  Partitions: {}", custom_partitions);
    mmap.write_parallel_with_partitions(
        Some(custom_partitions),
        |partition_id, chunk| {
            let len = chunk.len();
            for (i, byte) in chunk.iter_mut().enumerate() {
                *byte = ((partition_id * 64 + i) % 256) as u8;
            }
            println!(
                "  [Partition {}] Wrote {} bytes",
                partition_id, len
            );
            Ok(())
        },
    )?;
    mmap.flush()?;
    println!("✓ Pattern write completed and flushed");
    let file_size_on_disk = std::fs::metadata(file_path)?.len();
    println!(
        "✓ File size on disk: {} MB",
        file_size_on_disk / (1024 * 1024)
    );
    std::fs::remove_file(file_path).ok();
    Ok(())
}
