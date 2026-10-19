//! Windows kernel32 FFI bindings for file mapping and memory operations.

use std::ffi::c_void;

/// Windows file handle type.
pub type HANDLE = *mut c_void;

/// Windows boolean type.
pub type BOOL = i32;

/// Windows 32-bit unsigned integer type.
pub type DWORD = u32;

/// Windows 64-bit signed integer type.
pub type LARGE_INTEGER = i64;

/// Windows generic pointer type.
pub type LPVOID = *mut c_void;

/// Windows wide string pointer type (null-terminated UTF-16).
pub type LPCWSTR = *const u16;

/// Windows size type.
pub type SIZE_T = usize;

/// Invalid handle constant.
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as *mut c_void;

/// Memory protection flag: read and write access.
pub const PAGE_READWRITE: DWORD = 4;

/// File mapping access flag: read access.
pub const FILE_MAP_READ: DWORD = 4;

/// File mapping access flag: write access.
pub const FILE_MAP_WRITE: DWORD = 2;

/// File mapping access flag: execute access.
pub const FILE_MAP_EXECUTE: DWORD = 0x20;

/// Windows TRUE constant.
pub const TRUE: BOOL = 1;

/// Windows FALSE constant.
pub const FALSE: BOOL = 0;

/// Structure for setting file end of file information.
#[repr(C)]
pub struct FILE_END_OF_FILE_INFO {
    /// New end-of-file position.
    pub EndOfFile: LARGE_INTEGER,
}

/// File information class for end-of-file operations.
pub const FILE_END_OF_FILE_INFO_CLASS: DWORD = 20;

#[link(name = "kernel32")]
unsafe extern "system" {
    /// Creates a named or unnamed file mapping object.
    pub fn CreateFileMappingW(
        hFile: HANDLE,
        lpFileMappingAttributes: LPVOID,
        flProtect: DWORD,
        dwMaximumSizeHigh: DWORD,
        dwMaximumSizeLow: DWORD,
        lpName: LPCWSTR,
    ) -> HANDLE;

    /// Maps a view of a file mapping into the address space of a calling process.
    pub fn MapViewOfFileEx(
        hFileMappingObject: HANDLE,
        dwDesiredAccess: DWORD,
        dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD,
        dwNumberOfBytesToMap: SIZE_T,
        lpBaseAddress: LPVOID,
    ) -> LPVOID;

    /// Unmaps a mapped view of a file.
    pub fn UnmapViewOfFile(lpBaseAddress: LPVOID) -> BOOL;

    /// Closes an open object handle.
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;

    /// Sets file information by handle.
    pub fn SetFileInformationByHandle(
        hFile: HANDLE,
        FileInformationClass: DWORD,
        lpFileInformation: LPVOID,
        dwBufferSize: DWORD,
    ) -> BOOL;

    /// Flushes the view of a file mapping to the disk.
    pub fn FlushViewOfFile(lpBaseAddress: LPVOID, dwNumberOfBytesToFlush: SIZE_T) -> BOOL;
}
