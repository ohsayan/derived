//! This module illustrates the use of custom types that are `Copy`

use derived::{Ctor, Gtor};

#[derive(Clone, Copy, Ctor)]
pub struct Copyable {
    val_a: u8,
    val_b: u8,
}

#[derive(Ctor, Gtor)]
pub struct Holder {
    // tell the macro that this type is copyable
    #[gtor_copy]
    copyable: Copyable,
}

#[test]
fn test_holder() {
    let copyable = Copyable::new(1, 2);
    let holder = Holder::new(copyable);
    let y = holder.copyable;
    // notice the copy above ^^ (copyable is copied to holder, while the original remains)
    assert_eq!(y.val_a, copyable.val_b);
}
