use derived::Ctor;

#[derive(Ctor)]
pub struct User {
    name: String,
    email: String,
    verified: bool,
}

fn main() {
    let x = User::new("sayan".to_string(), "ohsayan@outlook.com".to_string(), true);
}
