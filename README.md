### Key Safety & Error-Handling Features
| Aspect | Implementation |
|--------|----------------|
| Input validation | Every FFI wrapper validates parameters (null pointers, invalid file descriptors, zero sizes) before calling unsafe code. |
| Error context | `anyhow::Context` chains errors with clear descriptions of which API failed and why. |
| Lifetime safety | `Arc<MmapMut>` ensures the mmap stays alive during parallel writes; Drop impl handles cleanup.
| Panic safety | `thread::join()` converts panics into `anyhow::Error` with `.map_err().` |
| Safe slice access | write_parallel constructs mutable slices from validated pointers; Rust's borrow checker prevents overlaps. |
| Platform abstraction | #[cfg(unix)] and #[cfg(windows)] conditionally compile OS-specific FFI. |
| Resource cleanup | All handles, file mappings, and pointers are properly closed/unmapped in Drop impls. |

This design encapsulates all unsafe code in small, auditable wrappers and propagates errors through anyhow for ergonomic error handling throughout the library.

