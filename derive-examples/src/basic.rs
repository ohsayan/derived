//! This module provides examples of how we can use the macros in a "holistic" manner
//!

use derived::{Ctor, Gtor, Stor};

#[derive(Ctor, Gtor, Stor)]
#[ctor_const] // makes the constructor a const fn
#[gtor_const] // makes the getters constant
pub struct User {
    name: &'static str,
    email: &'static str,
    verified: bool,
    userid: u64,
    followers: u64,
}

// use the constant ctor
const _USER: User = User::new("sayan", "ohsayan@outlook.com", true, 1, u64::MAX);
const _UNAME: &'static str = _USER.get_name();

#[derive(Ctor)]
struct MyType<'a, T: ToString + Copy> {
    str_value: &'a T,
    tag: u8,
}

#[test]
fn test_basic() {
    let mut x = User::new("sayan", "ohsayan@outlook.com", true, 1, u64::MAX);
    assert_eq!(x.get_name(), "sayan");
    assert_eq!(x.get_email(), "ohsayan@outlook.com");
    assert!(x.get_verified());
    assert_eq!(x.get_userid(), 1);
    assert_eq!(x.get_followers(), u64::MAX);
    // oh no, I lost followers
    x.set_followers(u64::MAX / 2);
    let ty = MyType::new(&125u8, 0);
    assert_eq!(ty.str_value.to_string(), "125");
    assert_eq!(ty.tag, 0);
}
