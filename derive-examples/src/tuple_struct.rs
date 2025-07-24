use derived::Ctor;

#[derive(Ctor)]
pub struct ThisIsATupleStruct<T: AsRef<str>>(T, u64, u64, u64);
#[derive(Ctor)]
pub struct EmptyTupleStruct();

#[test]
fn tuple_struct() {
    let ts = ThisIsATupleStruct::new("hello", 10, 20, 30);
    assert_eq!(ts.0, "hello");
    assert_eq!(ts.1, 10);
    assert_eq!(ts.2, 20);
    assert_eq!(ts.3, 30);
}

#[test]
fn tuple_struct_base_case() {
    let _ = EmptyTupleStruct::new();
}
