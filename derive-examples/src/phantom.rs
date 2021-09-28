//! This module illustrates the skipping of ctors, stors and gtors for phantom fields
//!

use derived::{Ctor, Gtor, Stor};
use std::marker::PhantomData;

#[derive(Ctor, Gtor, Stor)]
pub struct PhantomStuff<'a> {
    #[phantom] // mark the field to be phantom
    a: PhantomData<&'a u8>,
    b: u8,
}

#[test]
fn test_phantom_field() {
    // see, we only need to pass the non-phantom field
    let phantom = PhantomStuff::new(10);
    assert_eq!(phantom.b, 10);
}
