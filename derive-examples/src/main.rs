use derived::{Ctor, Gtor, Stor};

#[derive(Ctor, Gtor, Stor)]
pub struct User {
    name: &'static str,
    email: &'static str,
    verified: bool,
    userid: u64,
    followers: u64,
}

fn main() {
    let mut x = User::new("sayan", "ohsayan@outlook.com", true, 1, u64::MAX);
    assert_eq!(x.get_name(), "sayan");
    assert_eq!(x.get_email(), "ohsayan@outlook.com");
    assert!(x.get_verified());
    assert_eq!(x.get_userid(), 1);
    assert_eq!(x.get_followers(), u64::MAX);
    // oh no, I lost followers
    x.set_followers(u64::MAX / 2);
}
