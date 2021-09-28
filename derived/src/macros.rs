//! Internal macros
//!

macro_rules! genset {
    ($($ty:ty),*) => {{
        let mut hs = ::std::collections::HashSet::new();
        $(hs.insert(stringify!($ty));)*
        hs
    }};
}

macro_rules! gen_typeset {
    ($($ty:ty),*) => {
        lazy_static::lazy_static! {
            pub static ref TYCOPY: ::std::collections::HashSet<&'static str> = {
                genset!($($ty),*)
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

macro_rules! err_if_subattr_on_primary_attr {
    ($callpos:literal, $($attr:ident in $attrs:expr),* $(,)*) => {
        $(if ok_else_ret!($crate::util::single_instance_of_attr(&$attrs, $attr)) {
            return ::syn::Error::new(
                ::syn::spanned::Spanned::span($attrs.last().unwrap()),
                format!("Error: Marking sub-attribute `{}` on the {} is invalid!", $attr, $callpos),
            )
            .into_compile_error()
            .into();
        })*
    };
}
