//! This module shows the usage for declaring constant default values (compile-time default impl)
//!

use derived::Constdef;

#[derive(Constdef)]
pub struct Constable {
    x: u8,
    boolean: bool,
    integer: i32,
    small_float: core::primitive::f32,
    big_float: std::primitive::f64,
    num_array: [u8; 10],
    bool_array: [bool; 20],
    float_array: [f32; 30],
    tuple: (u8, u16),
    nested_tuple: ((u8, u8), u16),
}

const CONSTABLE: Constable = Constable::default();

#[test]
fn test_const_default() {
    assert_eq!(CONSTABLE.x, 0);
    assert!(!CONSTABLE.boolean);
    assert_eq!(CONSTABLE.integer, 0);
    assert_eq!(CONSTABLE.num_array, [0; 10]);
    assert_eq!(CONSTABLE.bool_array, [false; 20]);
    assert_eq!(CONSTABLE.float_array, [0.0; 30]);
    assert_eq!(CONSTABLE.small_float, 0.0);
    assert_eq!(CONSTABLE.big_float, 0.0);
    assert_eq!(CONSTABLE.tuple, (0, 0));
    assert_eq!(CONSTABLE.nested_tuple, ((0, 0), 0));
}
