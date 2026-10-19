use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_builder_bytes.bin";
    let data = vec![42u8; 2048];
    let mmap = AllocMmap::builder(file_path, 2048)
        .build_and_write_bytes(&data)?;
    println!("✓ Builder created and wrote file: {}", file_path);
    println!("  File size: {} bytes", mmap.size());
    println!("  Data written: {} bytes", data.len());
    mmap.flush()?;
    println!("✓ Flushed to disk");
    std::fs::remove_file(file_path).ok();
    Ok(())
}
