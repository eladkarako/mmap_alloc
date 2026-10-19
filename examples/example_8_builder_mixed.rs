use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_builder_mixed.bin";
    let header = vec![0xFFu8; 64];
    let mmap = AllocMmap::builder(file_path, 4096)
        .with_partitions(4)
        .build()?;
    println!(
        "✓ Created file with builder and 4 partitions: {}",
        file_path
    );
    mmap.write_bytes(&header)?;
    println!("✓ Wrote header: {} bytes", header.len());
    mmap.write_parallel_with_partitions(
        Some(4),
        |partition_id, chunk| {
            let offset = chunk.as_ptr() as usize
                - mmap.as_slice().as_ptr() as usize;
            if offset >= header.len() {
                for (i, byte) in chunk.iter_mut().enumerate() {
                    *byte = ((partition_id * 64 + i) % 256) as u8;
                }
            }
            println!(
                "  [Partition {}] Wrote {} bytes",
                partition_id,
                chunk.len()
            );
            Ok(())
        },
    )?;
    println!("✓ Mixed writes completed");
    mmap.flush()?;
    println!("✓ Flushed to disk");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
