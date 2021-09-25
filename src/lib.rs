use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::DataStruct;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
    let mut field_names = HashMap::new();
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident.clone();
    let fields = match &parsed_input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return syn::Error::new_spanned(
                parsed_input,
                "`#[derive(derived::Ctor)]` can only be used on structs",
            )
            .into_compile_error()
            .into()
        }
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
        fields.iter().for_each(|field| {
            let ty = field.ty.clone();
            let fname = field.ident.as_ref().unwrap().clone();
            field_names.insert(fname, ty);
        });
        let mut tokens = quote! {};
        let mut self_args = quote! {};
        for (fname, ty) in field_names {
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
