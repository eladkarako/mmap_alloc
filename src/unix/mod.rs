use crate::mmap::MmapMut;
use crate::unix_ffi;
use std::fs::File;

pub fn allocate_file(
    file: &File,
    size: u64,
) -> Result<()> {
    unix_ffi::safe_fallocate(file, size)
}

pub fn mmap_file(
    file: &File,
    size: usize,
) -> Result<MmapMut> {
    MmapMut::map_mut(file, size)
}

#[cfg(test)]
mod tests;
