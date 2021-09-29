use crate::util;
use ::proc_macro::TokenStream;
use ::quote::{quote, ToTokens};
use ::syn::{parse_macro_input, DeriveInput, Type};

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

macro_rules! gen_defset {
    ($($ty:ty => $defexpr:expr),*) => {
        lazy_static::lazy_static! {
            static ref CONSTDEF: ::std::collections::HashMap<&'static str, DefExpr> = {
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

pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = ok_else_ret!(util::get_struct_field_names(&ast));
    if fields.is_empty() {
        return {
            quote! {
                impl #impl_gen #struct_name #ty_gen #where_clause {
                    pub const fn default() -> Self {
                        Self {}
                    }
                }
                impl #impl_gen ::core::default::Default for #struct_name #ty_gen #where_clause {
                    fn default() -> Self {
                        Self::default()
                    }
                }
            }
        }
        .into();
    } else {
        let mut self_args = quote! {};
        for (ident, ty, _attrs) in fields {
            let is_const_able = match &ty {
                Type::Path(t) => {
                    if t.path.segments.len() == 1 {
                        // one segment, so we may have this
                        if t.path.segments[0].arguments.is_empty() {
                            // no path args, so this is definitely something we may have
                            let type_str = t.clone().into_token_stream().to_string();
                            CONSTDEF.get(type_str.as_str()).cloned()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Type::Array(arr) => {
                    let array_len = arr.len.clone().into_token_stream().to_string();
                    let elem = match &*arr.elem {
                        Type::Path(t) => {
                            if t.path.segments.len() == 1 {
                                // one segment, so we may have this
                                if t.path.segments[0].arguments.is_empty() {
                                    // no path args, so this is definitely something we may have
                                    let type_str = t.clone().into_token_stream().to_string();
                                    CONSTDEF.get(type_str.as_str()).cloned()
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    match elem {
                        Some(token) => {
                            let r;
                            r = match token {
                                DefExpr::Boolean => Some(DefExpr::BooleanArray(array_len)),
                                DefExpr::Char => Some(DefExpr::CharArray(array_len)),
                                DefExpr::Unit => Some(DefExpr::UnitArray(array_len)),
                                DefExpr::Numeric => Some(DefExpr::NumericArray(array_len)),
                                DefExpr::Float => Some(DefExpr::FloatingArray(array_len)),
                                _ => None,
                            };
                            r
                        }
                        None => None,
                    }
                }
                Type::Tuple(tp) if tp.elems.is_empty() => CONSTDEF.get("()").cloned(),
                _ => None,
            };
            let ret = match is_const_able {
                None => {
                    return syn::Error::new(
                        ident.span(),
                        "Error: This item cannot be evaluated at compile time",
                    )
                    .into_compile_error()
                    .into()
                }
                Some(texpr_ty) => texpr_ty,
            };
            match ret {
                DefExpr::Boolean => {
                    self_args = quote! {
                        #self_args
                        #ident: false,
                    }
                }
                DefExpr::Char => {
                    self_args = quote! {
                        #self_args
                        #ident: '\0',
                    }
                }
                DefExpr::Float => {
                    self_args = quote! {
                        #self_args
                        #ident: 0.0,
                    }
                }
                DefExpr::Numeric => {
                    self_args = quote! {
                        #self_args
                        #ident: 0,
                    }
                }
                DefExpr::Unit => {
                    self_args = quote! {
                        #self_args
                        #ident: (),
                    }
                }
                DefExpr::NumericArray(len) => {
                    let len: quote::__private::TokenStream = len.parse().unwrap();
                    self_args = quote! {
                        #self_args
                        #ident: [0; #len],
                    };
                }
                DefExpr::BooleanArray(len) => {
                    let len: quote::__private::TokenStream = len.parse().unwrap();
                    self_args = quote! {
                        #self_args
                        #ident: [false; #len],
                    };
                }
                DefExpr::CharArray(len) => {
                    let len: quote::__private::TokenStream = len.parse().unwrap();
                    self_args = quote! {
                        #self_args
                        #ident: ['\0'; #len],
                    };
                }
                DefExpr::FloatingArray(len) => {
                    let len: quote::__private::TokenStream = len.parse().unwrap();
                    self_args = quote! {
                        #self_args
                        #ident: [0.0; #len],
                    };
                }
                DefExpr::UnitArray(len) => {
                    let len: quote::__private::TokenStream = len.parse().unwrap();
                    self_args = quote! {
                        #self_args
                        #ident: [(); #len],
                    };
                }
            }
        }
        let tokens = quote! {
            impl #impl_gen #struct_name #ty_gen #where_clause {
                pub const fn default() -> Self {
                    Self {
                        #self_args
                    }
                }
            }
            impl #impl_gen ::core::default::Default for #struct_name #ty_gen #where_clause {
                fn default() -> Self {
                    Self::default()
                }
            }
        };
        tokens.into()
    }
}
