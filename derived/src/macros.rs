//! Internal macros
//!

macro_rules! gen_typeset {
    ($($ty:ty),*) => {
        lazy_static::lazy_static! {
            pub static ref TYCOPY: ::std::collections::HashSet<&'static str> = {
                let mut hs = ::std::collections::HashSet::new();
                $(
                    hs.insert(stringify!($ty));
                )*
                hs
            };
        }
    };
}

macro_rules! ok_else_ret {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return e,
        }
    };
}
