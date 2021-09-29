use crate::util;
use ::proc_macro::TokenStream;
use ::quote::{quote, ToTokens};
use ::syn::{parse_macro_input, DeriveInput, Type};

#[derive(Clone)]
pub enum DefExpr {
    Number,
    Boolean,
    Float,
    Char,
    Unit,
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
    u8 => DefExpr::Number,
    i8 => DefExpr::Number,
    u16 => DefExpr::Number,
    i16 => DefExpr::Number,
    u32 => DefExpr::Number,
    i32 => DefExpr::Number,
    u64 => DefExpr::Number,
    i64 => DefExpr::Number,
    u128 => DefExpr::Number,
    i128 => DefExpr::Number,
    bool => DefExpr::Boolean,
    usize => DefExpr::Number,
    isize => DefExpr::Number,
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
                    let type_str = t.clone().into_token_stream().to_string();
                    CONSTDEF.get(type_str.as_str()).cloned()
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
                DefExpr::Number => {
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
