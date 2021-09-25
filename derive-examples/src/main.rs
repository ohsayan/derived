use derived::Ctor;

#[derive(Ctor)]
pub struct User {
    name: &'static str,
    email: &'static str,
    verified: bool,
    userid: u64,
    followers: u64,
}

fn main() {
    let x = User::new("sayan", "ohsayan@outlook.com", true, 1, u64::MAX);
    assert_eq!(x.name, "sayan");
    assert_eq!(x.email, "ohsayan@outlook.com");
    assert!(x.verified);
    assert_eq!(x.userid, 1);
    assert_eq!(x.followers, u64::MAX);
}
