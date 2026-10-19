// C:\Users\Elad\RustroverProjects\mmap_alloc\examples\example_write.rs
use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output.bin";
    let file_size = 4096;
    let mmap = AllocMmap::create(file_path, file_size)?;
    println!("✓ Created memory-mapped file: {}", file_path);
    println!("  File size: {} bytes", mmap.size());
    mmap.write_parallel(|partition_id, chunk| {
        println!(
            "  [Partition {}] Writing {} bytes...",
            partition_id,
            chunk.len()
        );
        chunk.fill(b'A' + partition_id as u8);
        Ok(())
    })?;
    mmap.flush()?;
    println!("✓ Data flushed to disk");
    let data = std::fs::read(file_path)?;
    println!("✓ Verification: file contains {} bytes", data.len());
    std::fs::remove_file(file_path).ok();
    Ok(())
}
