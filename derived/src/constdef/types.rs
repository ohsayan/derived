use ::quote::quote;
use ::syn::Ident;

#[derive(Clone)]
pub enum DefExpr {
    Numeric,
    Boolean,
    Float,
    Char,
    Unit,
    CustomTuple(String),
    CustomArray(String),
}

impl DefExpr {
    /// Returns the base token's default value
    pub(super) fn into_base_token(self) -> quote::__private::TokenStream {
        match self {
            DefExpr::Boolean => {
                quote! {
                    false
                }
            }
            DefExpr::Char => {
                quote! {
                    '\0'
                }
            }
            DefExpr::Float => {
                quote! {
                    0.0
                }
            }
            DefExpr::Numeric => {
                quote! {
                    0
                }
            }
            DefExpr::Unit => {
                quote! {
                    ()
                }
            }
            DefExpr::CustomTuple(custom) => {
                let custom: quote::__private::TokenStream = custom.parse().unwrap();
                quote! {
                    (#custom)
                }
            }
            DefExpr::CustomArray(custom) => {
                let custom: quote::__private::TokenStream = custom.parse().unwrap();
                quote! {
                    [#custom]
                }
            }
        }
    }
    /// Returns tokens that finally resolves to `field: expr,`
    pub(super) fn into_tokens(self, ident: &Ident) -> quote::__private::TokenStream {
        let ret = self.into_base_token();
        quote! {
            #ident: #ret,
        }
    }
}

macro_rules! gen_defset {
    ($($ty:ty => $defexpr:expr),*) => {
        lazy_static::lazy_static! {
            pub(super) static ref CONSTDEF: ::std::collections::HashMap<&'static str, DefExpr> = {
                let mut hm = ::std::collections::HashMap::new();
                $(
                    hm.insert(stringify!($ty), $defexpr);
                )*
                hm
            };
        }
    };
}

gen_defset! {
    u8 => DefExpr::Numeric,
    i8 => DefExpr::Numeric,
    u16 => DefExpr::Numeric,
    i16 => DefExpr::Numeric,
    u32 => DefExpr::Numeric,
    i32 => DefExpr::Numeric,
    u64 => DefExpr::Numeric,
    i64 => DefExpr::Numeric,
    u128 => DefExpr::Numeric,
    i128 => DefExpr::Numeric,
    bool => DefExpr::Boolean,
    usize => DefExpr::Numeric,
    isize => DefExpr::Numeric,
    char => DefExpr::Char,
    f32  => DefExpr::Float,
    f64  => DefExpr::Float,
    () => DefExpr::Unit
}
