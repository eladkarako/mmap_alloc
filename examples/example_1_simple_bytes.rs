use mmap_alloc::AllocMmap;
fn main() -> anyhow::Result<()> {
    let file_path = "output_simple_bytes.bin";
    let data = b"Hello, this is a simple byte write!";
    let mmap = AllocMmap::create(file_path, 1024)?;
    println!("✓ Created file: {}", file_path);
    mmap.write_bytes(data)?;
    println!("✓ Wrote {} bytes", data.len());
    mmap.flush()?;
    println!("✓ Flushed to disk");
    let read_data = std::fs::read(file_path)?;
    println!("✓ Verification: {} bytes on disk", read_data.len());
    std::fs::remove_file(file_path).ok();
    Ok(())
}
