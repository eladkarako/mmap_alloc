use crate::bindings::windows::*;

#[test]
fn test_constants_defined() {
    // Verify Windows constants are accessible
    assert_eq!(PAGE_READWRITE, 4);
    assert_eq!(FILE_MAP_READ, 4);
    assert_eq!(FILE_MAP_WRITE, 2);
    assert_eq!(TRUE, 1);
    assert_eq!(FALSE, 0);
}

#[test]
fn test_invalid_handle_value() {
    // Ensure INVALID_HANDLE_VALUE is -1
    assert_eq!(INVALID_HANDLE_VALUE as isize, -1isize);
}
