use crate::util;
use ::proc_macro::TokenStream;
use ::quote::quote;
use ::syn::{parse_macro_input, DeriveInput, Type};
// internal modules
mod type_analysis;
mod types;
use self::types::CONSTDEF;

pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = ok_else_ret!(util::get_struct_field_names(&ast));
    if fields.is_empty() {
        // fast case: empty struct
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
        // extended case: struct with fields
        let mut self_args = quote! {};
        for (ident, ty, _attrs) in fields {
            let is_const_able = match &ty {
                Type::Path(t) => self::type_analysis::analyze_type_path(t),
                Type::Array(arr) => self::type_analysis::process_array(arr),
                Type::Tuple(tp) if tp.elems.is_empty() => CONSTDEF.get("()").cloned(),
                Type::Tuple(tpl) => self::type_analysis::recursive_process_tuple(tpl),
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
            let r = ret.into_tokens(ident);
            self_args = quote! {
                #self_args
                #r
            };
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
