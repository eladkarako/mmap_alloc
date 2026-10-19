use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_builder_custom_partitions.bin";
    let partitions = 3;
    let mmap = AllocMmap::builder(file_path, 6144)
        .with_partitions(partitions)
        .build()?;
    println!(
        "✓ Built file with {} partitions: {}",
        partitions, file_path
    );
    mmap.write_parallel_with_partitions(
        Some(partitions),
        |partition_id, chunk| {
            chunk.fill(b'P' + (partition_id as u8));
            println!(
                "  [Partition {}] Wrote {} bytes",
                partition_id,
                chunk.len()
            );
            Ok(())
        },
    )?;
    println!("✓ Parallel write with custom partitions completed");
    mmap.flush()?;
    println!("✓ Flushed to disk");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
