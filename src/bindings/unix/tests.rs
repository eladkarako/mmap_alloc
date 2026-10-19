use crate::bindings::unix::*;

#[test]
fn test_constants_defined() {
    // Verify constants are accessible and have expected values
    assert_eq!(PROT_READ, 1);
    assert_eq!(PROT_WRITE, 2);
    assert_eq!(MAP_SHARED, 1);
    assert_eq!(MS_SYNC, 4);
    assert_eq!(MS_ASYNC, 1);
}

#[test]
fn test_map_failed_constant() {
    // Ensure MAP_FAILED is -1 as expected
    assert_eq!(MAP_FAILED as isize, -1isize);
}
