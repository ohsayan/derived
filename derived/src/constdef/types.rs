use ::quote::quote;
use ::syn::Ident;

#[derive(Clone)]
pub enum DefExpr {
    Numeric,
    Boolean,
    Float,
    Char,
    Unit,
    NumericArray(String),
    BooleanArray(String),
    FloatingArray(String),
    CharArray(String),
    UnitArray(String),
}

impl DefExpr {
    pub(super) fn get_simple_array(ty: Self, expr: String) -> Option<Self> {
        let r = match ty {
            DefExpr::Numeric => Self::NumericArray(expr),
            DefExpr::Boolean => Self::BooleanArray(expr),
            DefExpr::Float => Self::FloatingArray(expr),
            DefExpr::Char => Self::CharArray(expr),
            DefExpr::Unit => Self::UnitArray(expr),
            _ => return None,
        };
        Some(r)
    }
    pub(super) fn into_tokens(self, ident: &Ident) -> quote::__private::TokenStream {
        match self {
            DefExpr::Boolean => {
                quote! {
                    #ident: false,
                }
            }
            DefExpr::Char => {
                quote! {
                    #ident: '\0',
                }
            }
            DefExpr::Float => {
                quote! {
                    #ident: 0.0,
                }
            }
            DefExpr::Numeric => {
                quote! {
                    #ident: 0,
                }
            }
            DefExpr::Unit => {
                quote! {
                    #ident: (),
                }
            }
            DefExpr::NumericArray(len) => {
                let len: quote::__private::TokenStream = len.parse().unwrap();
                quote! {
                    #ident: [0; #len],
                }
            }
            DefExpr::BooleanArray(len) => {
                let len: quote::__private::TokenStream = len.parse().unwrap();
                quote! {
                    #ident: [false; #len],
                }
            }
            DefExpr::CharArray(len) => {
                let len: quote::__private::TokenStream = len.parse().unwrap();
                quote! {
                    #ident: ['\0'; #len],
                }
            }
            DefExpr::FloatingArray(len) => {
                let len: quote::__private::TokenStream = len.parse().unwrap();
                quote! {
                    #ident: [0.0; #len],
                }
            }
            DefExpr::UnitArray(len) => {
                let len: quote::__private::TokenStream = len.parse().unwrap();
                quote! {
                    #ident: [(); #len],
                }
            }
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
