use derived::Ctor;

#[derive(Ctor)]
pub struct ThisIsATupleStruct<T: AsRef<str>>(T, u64);

#[test]
fn tuple_struct() {
    let ts = ThisIsATupleStruct::new("hello", 10);
    assert_eq!(ts.0, "hello");
    assert_eq!(ts.1, 10);
}
