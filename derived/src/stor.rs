//! This module is responsible for the actual generation of the stor
//!

use crate::util;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

pub(crate) fn derive_stor(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = ok_else_ret!(util::get_struct_field_names(&ast));
    if !fields.is_empty() {
        let mut q = quote!();
        for (field, ty, _) in fields {
            let field_name_str = field.to_string();
            let mut fname = "set_".to_owned();
            fname.push_str(&field_name_str);
            let doc_comment = format!(
                "Sets the value for the `{field}` field in struct [`{struct_name}`]",
                struct_name = struct_name,
                field = field_name_str
            );
            let fname = Ident::new(&fname, field.span());
            q = quote! {
                #q
                #[doc = #doc_comment]
                pub fn #fname(&mut self, #field: #ty) {
                    self.#field = #field;
                }
            };
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
