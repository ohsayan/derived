//! This module is responsible for the actual generation of the gtor
//!

use crate::util;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident, Type};
use util::TYCOPY;

/// The attribute for constant (compile-time) getters
const ATTR_CONST_GTOR: &str = "const_gtor";

pub(crate) fn derive_gtor(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();

    // get generics
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    // get fields
    let fields = match util::get_struct_field_names(&ast) {
        Ok(f) => f,
        Err(e) => return e,
    };
    // get function header
    let func = match util::get_func_header(&ast.attrs, ATTR_CONST_GTOR) {
        Ok(f) => f,
        Err(e) => return e,
    };
    if !fields.is_empty() {
        let mut q = quote!();
        for (field, ty) in fields {
            let field_name_str = field.to_string();
            let mut fname = "get_".to_owned();
            fname.push_str(&field_name_str);
            let doc_comment = format!(
                "Returns the value for the `{field}` field in struct [`{struct_name}`]",
                struct_name = struct_name,
                field = field_name_str
            );
            let fname = Ident::new(&fname, field.span());

            let is_prim = match &ty {
                Type::Path(t) => {
                    let type_str = t.clone().into_token_stream().to_string();
                    TYCOPY.contains(type_str.as_str())
                }
                // all these are copy type (fnptrs, ptrs, refs); no point in returning another ref
                Type::BareFn(_) | Type::Never(_) | Type::Ptr(_) | Type::Reference(_) => true,
                _ => false,
            };

            if is_prim {
                // a copy-able type
                q = quote! {
                    #q
                    #[doc = #doc_comment]
                    #func #fname(&self) -> #ty {
                        self.#field
                    }
                };
            } else {
                q = quote! {
                    #q
                    #[doc = #doc_comment]
                    #func #fname(&self) -> &#ty {
                        &self.#field
                    }
                };
            }
        }
        q = quote! {
            impl #impl_gen #struct_name #ty_gen #where_clause {
                #q
            }
        };
        q.into()
    } else {
        return "".parse().unwrap();
    }
}
