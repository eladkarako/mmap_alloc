use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_mixed_writes.bin";
    let file_size: u64 = 4096;
    let header = b"HEADER";
    let mmap = AllocMmap::create(file_path, file_size)?;
    println!("✓ Created file: {}", file_path);
    mmap.write_bytes(header)?;
    println!("✓ Wrote header: {} bytes", header.len());
    mmap.write_parallel(|partition_id, chunk| {
        let offset = chunk.as_ptr() as usize
            - mmap.as_slice().as_ptr() as usize;
        for (i, byte) in chunk.iter_mut().enumerate() {
            *byte = ((offset + i) % 256) as u8;
        }
        println!(
            "  [Partition {}] Wrote pattern {} bytes",
            partition_id,
            chunk.len()
        );
        Ok(())
    })?;
    println!("✓ Parallel pattern write completed");
    mmap.flush()?;
    println!("✓ Flushed to disk");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
