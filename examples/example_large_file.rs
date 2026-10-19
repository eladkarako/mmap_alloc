// C:\Users\Elad\RustroverProjects\mmap_alloc\examples\example_large_file.rs
use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_large.bin";
    let file_size: u64 = 10 * 1024 * 1024;
    println!(
        "Creating a {} MB memory-mapped file...",
        file_size / (1024 * 1024)
    );
    let mmap = AllocMmap::create(file_path, file_size)?;
    println!("✓ File created and pre-allocated");
    println!("  Path: {}", file_path);
    println!("  Size: {} MB", mmap.size() / (1024 * 1024));
    let cpu_count = num_cpus::get();
    println!(
        "  Using {} threads",
        (cpu_count).saturating_sub(1).max(1)
    );
    mmap.write_parallel(|partition_id, chunk| {
        let len = chunk.len();
        for (i, byte) in chunk.iter_mut().enumerate() {
            *byte = ((partition_id * 256 + i) % 256) as u8;
        }
        println!(
            "  [Partition {}] Wrote {} bytes",
            partition_id, len
        );
        Ok(())
    })?;
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
