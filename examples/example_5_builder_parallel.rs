use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_builder_parallel.bin";
    let mmap = AllocMmap::builder(file_path, 8192).build()?;
    println!("✓ Built file with builder: {}", file_path);
    mmap.write_parallel(|partition_id, chunk| {
        chunk.fill(b'X' + partition_id as u8);
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
