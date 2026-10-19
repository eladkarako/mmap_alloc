//! Platform-specific FFI bindings for memory mapping.
#![allow(non_snake_case, non_camel_case_types)]

#[cfg(unix)]
/// Unix/Linux FFI bindings.
pub mod unix;

#[cfg(windows)]
/// Windows FFI bindings.
pub mod windows;
