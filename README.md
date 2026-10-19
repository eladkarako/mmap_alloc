## `mmap_alloc`

tiny, fast and safe file writer crate, that parallel writes to a pre-allocated memory-mapped file.

<hr/>

### examples

```txt
mmap_alloc/
├── examples/
│   ├── basic_write.rs
│   ├── parallel_write.rs
│   └── large_file.rs
├── src/
├── Cargo.toml
└── ...
```

1. Basic Write Example - `cargo run --example  example_basic_write`
2. Parallel Write Example - `cargo run --example  example_parallel_write --features multithreaded`
3. Large File Example - `cargo run --example  example_large_file`

These examples showcase the crate's key features:  
- pre-allocation efficiency.
- simple synchronous writes.
- parallel writes with the optional feature.
- async flushing.

Each includes a little bit of a rustdocs as well.

<hr/>

