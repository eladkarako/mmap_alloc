pub mod allocate;
pub mod mmap;
pub mod writer;

// Re-export the main public API
pub use mmap::MmapMut;
pub use writer::{AllocMmap, AllocMmapBuilder};
