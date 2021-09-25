use derived::Ctor;

#[test]
fn test_basic() {
    #[derive(Ctor)]
    struct Loader {
        a: u16,
        b: u16,
    }
    let ld = Loader::new(5, 10);
    assert_eq!(ld.a, 5);
    assert_eq!(ld.b, 10);
}

#[test]
fn test_empty_struct() {
    #[derive(Ctor)]
    struct Empty {}
    let _ = Empty::new();
}

#[test]
fn test_multiple_types() {
    #[derive(Ctor)]
    struct MType {
        name: &'static str,
        verified: bool,
        userid: u64,
    }
    let mt = MType::new("Sayan", true, 1);
    assert_eq!(mt.name, "Sayan");
    assert_eq!(mt.verified, true);
    assert_eq!(mt.userid, 1);
}
