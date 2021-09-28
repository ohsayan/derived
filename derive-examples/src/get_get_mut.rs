use derived::{Ctor, Gtor};

#[derive(Ctor, Gtor)]
#[gtor(get, get_mut)]
pub struct Mutable {
    x_axis: u8,
    y_axis: u8,
}

#[test]
fn test_get_and_get_mut() {
    let mut m = Mutable::new(0, 0);
    // move x by 1 unit
    *m.get_x_axis_mut() = 1;
    // move y by 2 units
    *m.get_y_axis_mut() = 2;
    assert_eq!(m.get_x_axis(), 1);
    assert_eq!(m.get_y_axis(), 2);
}
