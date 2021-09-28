//! This module is responsible for the actual generation of the ctor
//!

use crate::util;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

const ATTR_CONST_CTOR: &str = "ctor_const";

pub fn derive_ctor(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = match util::get_struct_field_names(&ast) {
        Ok(f) => f,
        Err(e) => return e,
    };
    let func = ok_else_ret!(util::get_func_header(&ast.attrs, ATTR_CONST_CTOR));
    if fields.is_empty() {
        // handle fast case: empty struct
        return {
            quote! {
                impl #impl_gen #struct_name #ty_gen #where_clause {
                    #func new() -> Self {
                        Self {}
                    }
                }
            }
        }
        .into();
    } else {
        // handle extended case: struct with fields
        let mut tokens = quote! {};
        let mut self_args = quote! {};
        for (fname, ty, _) in fields {
            tokens = quote! {
                #tokens
                #fname: #ty,
            };
            self_args = quote! {
                #self_args
                #fname,
            }
        }
        let tokens = quote! {
            impl #impl_gen #struct_name #ty_gen #where_clause {
                #func new(
                    #tokens
                ) -> #struct_name #ty_gen {
                    Self {
                        #self_args
                    }
                }
            }
        };
        tokens.into()
    }
}
