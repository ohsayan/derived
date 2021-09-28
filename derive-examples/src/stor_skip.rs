//! This module shows us how we can skip the creation of setters for certain fields
//!

use derived::{Ctor, Stor};

#[derive(Ctor, Stor)]
struct X {
    a: u8,
    #[stor_skip] // no setters will be generated for this field
    unsettable: u8,
}

#[test]
fn test_stor_skip() {
    let mut x = X::new(10, 20);
    assert_eq!(x.a, 10);
    assert_eq!(x.unsettable, 20);
    x.set_a(11);
    // uncomment to error:
    // x.set_unsettable(21);
}
