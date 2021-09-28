//! This module is responsible for the actual generation of the ctor
//!

use crate::util;
use crate::util::ATTR_PHANTOM;
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
    err_if_subattr_on_primary_attr!(
        "entire struct",
        // a struct cannot be entirely phantom
        ATTR_PHANTOM in ast.attrs,
    );
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
        for (fname, ty, attrs) in fields {
            err_if_subattr_on_primary_attr!(
                "field",
                // marking const_ctor on a field is invalid
                ATTR_CONST_CTOR in attrs,
            );
            let is_phantom = ok_else_ret!(util::single_instance_of_attr(attrs, util::ATTR_PHANTOM));
            if !is_phantom {
                // not a phantomdata struct, add it
                tokens = quote! {
                    #tokens
                    #fname: #ty,
                };
                self_args = quote! {
                    #self_args
                    #fname,
                };
            } else {
                self_args = quote! {
                    #self_args
                    #fname: ::core::marker::PhantomData,
                };
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
