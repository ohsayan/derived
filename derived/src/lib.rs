//! `derived`: A macro to automate the boring stuff
//!
//! The `derived` crate aims to provided macros to automate boring things like writing functions
//! for constructors. Just look at the list of available macros and you'll find an example for each.
//!
//! Available macros:
//! - [`Ctor`]: Generate constructors automatically
//! - [`Gtor`]: Generate getters automatically
//!

use proc_macro::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::{Ident, Type};

fn get_struct_field_names(parsed_input: &DeriveInput) -> Result<Vec<(Ident, Type)>, TokenStream> {
    let fields = match &parsed_input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(syn::Error::new_spanned(
                parsed_input,
                "`#[derive(derived::Ctor)]` can only be used on structs",
            )
            .into_compile_error()
            .into());
        }
    };
    if fields.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(fields
            .iter()
            .map(|field| {
                let ty = field.ty.clone();
                let fname = field.ident.as_ref().unwrap().clone();
                (fname, ty)
            })
            .collect())
    }
}

#[proc_macro_derive(Ctor)]
/// # Ctor: Get a constructor derived
///
/// The [`Ctor`] macro will take the fields in the order they are declared and generate a
/// constructor, that is a `YourStruct::new()` function.
///
/// ## Example
/// ```
/// use derived::Ctor;
///
/// #[derive(Ctor)]
/// struct MyStruct {
///     int: u32,
///     unsigned_int: i32,
/// }
///
/// let ms = MyStruct::new(1, -1);
/// assert_eq!(ms.int, 1);
/// assert_eq!(ms.unsigned_int, -1);
/// ```
pub fn derive_ctor(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident.clone();
    let fields = match get_struct_field_names(&parsed_input) {
        Ok(f) => f,
        Err(e) => return e,
    };
    if fields.is_empty() {
        // handle fast case: empty struct
        return {
            quote! {
                impl #struct_name {
                    pub fn new() -> Self {
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
        for (fname, ty) in fields {
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
            impl #struct_name {
                pub fn new(
                    #tokens
                ) -> #struct_name {
                    Self {
                        #self_args
                    }
                }
            }
        };
        tokens.into()
    }
}

#[proc_macro_derive(Gtor)]
/// # Gtor: Get the getters derived
///
/// Gtor takes the fields in order and generates getters for each field. For example,
/// if you have fields named `userid` and `name`, then the getters generated will be
/// `get_userid` and `get_name`, returning references to the appropriate types. In other
/// words, `get_*` named methods will be derived per your fields
///
/// ## Example
/// ```
/// use derived::Gtor;
/// #[derive(Gtor)]
/// struct MyStruct {
///     name: String,
///     userid: u64,
/// }
///
/// let ms = MyStruct { name: "Sayan".to_owned(), userid: 16 };
/// assert_eq!(ms.get_name(), "Sayan");
/// ```
pub fn derive_gtor(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident.clone();
    let fields = match get_struct_field_names(&parsed_input) {
        Ok(f) => f,
        Err(e) => return e,
    };
    if !fields.is_empty() {
        let mut q = quote!();
        for (field, ty) in fields {
            let mut fname = "get_".to_owned();
            fname.push_str(&field.to_string());
            let fname = Ident::new(&fname, field.span());
            q = quote! {
                #q
                pub fn #fname(&self) -> &#ty {
                    &self.#field
                }
            };
        }
        q = quote! {
            impl #struct_name {
                #q
            }
        };
        q.into()
    } else {
        return "".parse().unwrap();
    }
}
