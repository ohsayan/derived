use derived::{Ctor, Gtor};

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
    struct Empty1 {}
    #[derive(Ctor)]
    struct Empty2();
    let _ = Empty1::new();
    let _ = Empty2::new();
}

#[test]
fn test_tuple_ctor_fancy_types() {
    #[derive(Ctor)]
    struct Fancy<T, U: AsRef<str>>(Option<T>, u64, U);
    let fancy = Fancy::new(Some("hello".to_owned()), 0, "world");
    assert_eq!(fancy.0.as_deref(), Some("hello"));
    assert_eq!(fancy.1, 0);
    assert_eq!(fancy.2, "world");
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

#[test]
fn test_generics_ctor() {
    #[derive(Ctor)]
    struct MyStruct<T: ToString> {
        value: T,
        id: u64,
    }
    let x: MyStruct<f32> = MyStruct::new(10.23, 1);
    assert_eq!(x.value.to_string(), "10.23");
    assert_eq!(x.id, 1);
}

#[test]
fn test_generics_gtor() {
    #[derive(Ctor, Gtor)]
    struct MyStruct<T: ToString> {
        value: T,
        id: u64,
    }
    let x: MyStruct<f32> = MyStruct::new(10.23, 1);
    // need a reference because get_value can't "look into" generic parameters to know if
    // they're copy types
    assert_eq!(x.get_value(), &10.23);
    assert_eq!(x.get_id(), 1);
}
