## `mmap_alloc` - Rust crate.

an extremely fast file writer. it pre-allocates a memory-mapped file, then parallel write it before flushing it to disk.

by default it uses the amount of CPUs available minus one,  
so for 8 CPUs, 7 threads will be used, and the write load will be partitioned to 7 parallel writing thread,  
you may specify you explicit amount of desired parallel write threads.  

<hr/>

### examples

```txt
mmap_alloc/
├── examples/
│   ├── example_write.rs
│   └── example_large_file.rs
├── src/
├── Cargo.toml
└── ...
```

1. Basic Write Example - `cargo run --example  example_write`
2. Large File Example - `cargo run --example  example_large_file`

with (optionally):  
`--target x86_64-pc-windows-msvc`  
`--target x86_64-unknown-linux-gnu`  

These examples showcase the crate's key features:  
- pre-allocation efficiency.
- simple synchronous writes.
- parallel writes with the optional feature.
- async flushing.

Each includes a little bit of a rustdocs as well.

<hr/>

