use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_builder_partitions_bytes.bin";
    let data: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
    let mmap = AllocMmap::builder(file_path, 4096)
        .with_partitions(2)
        .build_and_write_bytes(&data)?;
    println!(
        "✓ Builder with 2 partitions created and wrote file: {}",
        file_path
    );
    println!("  File size: {} bytes", mmap.size());
    mmap.flush()?;
    println!("✓ Flushed to disk");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
