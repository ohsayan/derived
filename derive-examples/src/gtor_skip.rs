use derived::{Ctor, Gtor};

#[derive(Ctor, Gtor)]
struct TwoFieldStruct {
    a: u8,
    #[gtor_skip] // no getters will be generated for this field
    getless: bool,
}

#[test]
fn test_gtor_skip() {
    let tfs = TwoFieldStruct::new(1, true);
    assert_eq!(tfs.get_a(), 1);
    assert!(tfs.getless);
    // uncomment to error:
    // assert!(tfs.get_getless());
}
