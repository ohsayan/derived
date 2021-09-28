//! This module is responsible for the actual generation of the gtor
//!

use crate::util;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Ident, Meta, Token, Type};
use util::TYCOPY;

/// The attribute for constant (compile-time) getters
const ATTR_CONST_GTOR: &str = "gtor_const";
const ATTR_GTOR_COPY: &str = "gtor_copy";
const ATTR_GTOR_SKIP: &str = "gtor_skip";

pub(crate) fn derive_gtor(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let mut attrlist: ::std::collections::HashSet<String> = Default::default();
    for attr in &ast.attrs {
        if attr.path.is_ident("gtor") {
            attrlist.extend(
                attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .unwrap()
                    .iter()
                    .map(|v| v.path().into_token_stream().to_string()),
            );
            break;
        }
    }
    let needs_get = attrlist.get("get").map(|_| true).unwrap_or(true);
    let needs_get_mut = attrlist.get("get_mut").map(|_| true).unwrap_or(false);

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
        for (field, ty, attrs) in fields {
            let is_phantom = ok_else_ret!(util::single_instance_of_attr(attrs, util::ATTR_PHANTOM));
            let is_explicitly_copy =
                ok_else_ret!(util::single_instance_of_attr(attrs, ATTR_GTOR_COPY));
            let is_skipped = ok_else_ret!(util::single_instance_of_attr(attrs, ATTR_GTOR_SKIP));
            if (is_skipped && is_explicitly_copy) || (is_explicitly_copy && is_phantom) {
                // both at once, huh?
                return syn::Error::new(
                    field.span(),
                    "Using `#[gtor_copy]` with `#[gtor_skip]` is invalid",
                )
                .into_compile_error()
                .into();
            }
            if !(is_skipped && is_phantom) {
                // not skipped and not phantom, so add gtor
                let is_prim = match &ty {
                    Type::Path(t) => {
                        let type_str = t.clone().into_token_stream().to_string();
                        TYCOPY.contains(type_str.as_str())
                    }
                    // all these are copy type (fnptrs, ptrs, refs); no point in returning another ref
                    Type::BareFn(_) | Type::Never(_) | Type::Ptr(_) | Type::Reference(_) => true,
                    _ => false,
                };
                let field_name_str = field.to_string();

                if needs_get {
                    let mut fname = "get_".to_owned();
                    fname.push_str(&field_name_str);
                    let doc_comment = format!(
                        "Returns the value for the `{field}` field in struct [`{struct_name}`]",
                        struct_name = struct_name,
                        field = field_name_str
                    );
                    let fname = Ident::new(&fname, field.span());

                    if is_prim || is_explicitly_copy {
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
                if needs_get_mut {
                    let fname = format!("get_{field}_mut", field = field_name_str);
                    let doc_comment = format!(
                        "Returns a mutable reference to the `{field}` field in struct [`{struct_name}`]",
                        struct_name = struct_name,
                        field = field_name_str
                    );
                    let fname = Ident::new(&fname, field.span());

                    q = quote! {
                        #q
                        #[doc = #doc_comment]
                        pub fn #fname(&mut self) -> &mut #ty {
                            &mut self.#field
                        }
                    };
                }
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
