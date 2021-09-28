//! This module shows the usage for declaring constant default values (compile-time default impl)
//!

use derived::Constdef;

#[derive(Constdef)]
pub struct Constable {
    x: u8,
    boolean: bool,
    integer: i32,
}

const CONSTABLE: Constable = Constable::default();

#[test]
fn test_const_default() {
    assert_eq!(CONSTABLE.x, 0);
    assert!(!CONSTABLE.boolean);
    assert_eq!(CONSTABLE.integer, 0);
}
