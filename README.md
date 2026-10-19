## `mmap_alloc` - Rust crate.

an extremely fast file writer. it pre-allocates a memory-mapped file, then parallel write it before flushing it to disk.

by default it uses the amount of CPUs available minus one,  
so for 8 CPUs, 7 threads will be used, and the write load will be partitioned to 7 parallel writing thread,  
you may specify you explicit amount of desired parallel write threads.  

<hr/>

### examples
- example_1_simple_bytes
- example_2_builder_bytes
- example_3_builder_partitions_bytes
- example_4_create_parallel
- example_5_builder_parallel
- example_6_builder_custom_partitions_parallel
- example_7_combined_mixed_writes
- example_8_builder_mixed

you can test those like this: `cargo run --example  example_1_simple_bytes`  
optionally adding `--target x86_64-pc-windows-msvc` or `--target x86_64-unknown-linux-gnu` (use the toolchains you have).

<hr/>

