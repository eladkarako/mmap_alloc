use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_create_parallel.bin";
    let file_size: u64 = 8192;
    let mmap = AllocMmap::create(file_path, file_size)?;
    println!("✓ Created file: {}", file_path);
    mmap.write_parallel(|partition_id, chunk| {
        chunk.fill(b'A' + partition_id as u8);
        println!(
            "  [Partition {}] Wrote {} bytes",
            partition_id,
            chunk.len()
        );
        Ok(())
    })?;
    println!("✓ Parallel write completed");
    mmap.flush()?;
    println!("✓ Flushed to disk");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
