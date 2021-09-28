//! `derived`: A macro to automate the boring stuff
//!
//! The `derived` crate aims to provide macros to automate boring things like writing functions
//! for constructors. Just look at the list of available macros and you'll find an example for each.
//!
//! ## Features
//!
//! - [`Ctor`]: To generate constructors
//! - [`Gtor`]: To generate getters
//! - [`Stor`]: To generate setters
//! - Full lifetimes, generics and `where` clause support
//!

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{parse_macro_input, DeriveInput, Ident, Type};
mod util;

const ATTR_CONST_CTOR: &str = "const_ctor";
const ATTR_CONST_GTOR: &str = "const_gtor";

macro_rules! gen_typeset {
    ($($ty:ty),*) => {
        lazy_static::lazy_static! {
            static ref TYCOPY: HashSet<&'static str> = {
                let mut hs = HashSet::new();
                $(
                    hs.insert(stringify!($ty));
                )*
                hs
            };
        }
    };
}

gen_typeset! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, str, bool, usize, isize, char, f32, f64
}

#[proc_macro_derive(Ctor, attributes(const_ctor))]
/// # Ctor: Get a constructor derived
///
/// The [`Ctor`] macro will take the fields in the order they are declared and generate a
/// constructor, that is a `YourStruct::new()` function.
///
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
///
/// ## Constant constructors
///
/// To make your constructors `const`, simply add the `#[const_ctor]` attribute to the top
/// of your struct.
///
/// ### Example
///
/// ```
/// use derived::Ctor;
///
/// #[derive(Ctor)]
/// #[const_ctor]
/// pub struct MyConst {
///     a: u8,
///     b: u8,
/// }
/// // you can now use it in constant contexts
/// const MC: MyConst = MyConst::new(1, 2);
/// ```
///
pub fn derive_ctor(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = match util::get_struct_field_names(&ast) {
        Ok(f) => f,
        Err(e) => return e,
    };
    let func = match util::get_func_header(&ast.attrs, ATTR_CONST_CTOR) {
        Ok(f) => f,
        Err(_) => {
            return syn::Error::new_spanned(&ast, "Duplicate attributes found for `const_ctor`")
                .into_compile_error()
                .into()
        }
    };
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

#[proc_macro_derive(Gtor, attributes(const_gtor))]
/// # Gtor: Get the getters derived
///
/// Gtor takes the fields in order and generates getters for each field. For example,
/// if you have fields named `userid` and `name`, then the getters generated will be
/// `get_userid` and `get_name`, returning references to the appropriate types. In other
/// words, `get_*` named methods will be derived per your fields.
///
/// ## Important note
///
/// ### References
/// If any of the fields within the struct are primitive types that do not require large copies,
/// then the value is returned directly instead of a reference to it:
/// ```text
/// u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, str, bool, usize, isize, char, f32, f64
/// ```
///
/// ### Doc-comments
///
/// The [`Gtor`] macro will automatically add a doc comment of the form:
/// ```text
/// Returns the value for the `<struct_field>` field in struct [`<struct_name>`]
/// ```
///
/// ### Constant getters
///
/// If you need your getters to be `const` (to use it in constant contexts), you can simply
/// add the `#[const_gtor]` attribute.
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
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = match util::get_struct_field_names(&ast) {
        Ok(f) => f,
        Err(e) => return e,
    };
    let func = match util::get_func_header(&ast.attrs, ATTR_CONST_GTOR) {
        Ok(f) => f,
        Err(_) => {
            return syn::Error::new_spanned(&ast, "Duplicate attributes found for `const_gtor`")
                .into_compile_error()
                .into()
        }
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

#[proc_macro_derive(Stor)]
/// # Stor: Get the setters derived
///
/// Stor takes the fields in order and generates setters for each field. For example,
/// if you have fields named `userid` and `name`, then the setters generated will be
/// `set_userid` and `set_name`, accepting values for the appropriate types. In other
/// words, `set_*` named methods will be derived per your fields.
///
///
/// ## Doc-comments
///
/// The [`Stor`] macro will automatically add a doc comment of the form:
/// ```text
/// Sets the value for the `<struct_field>` field in struct [`<struct_name>`]
/// ```
///
/// ## Example
/// ```
/// use derived::Stor;
/// #[derive(Stor)]
/// struct MyStruct {
///     name: String,
///     userid: u64,
/// }
///
/// let mut ms = MyStruct { name: "Sayan".to_owned(), userid: 1 };
/// assert_eq!(ms.name, "Sayan");
/// assert_eq!(ms.userid, 1);
/// ms.set_userid(0);
/// assert_eq!(ms.userid, 0);
/// ```
pub fn derive_stor(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let struct_name = ast.ident.clone();
    let (impl_gen, ty_gen, where_clause) = &ast.generics.split_for_impl();
    let fields = match util::get_struct_field_names(&ast) {
        Ok(f) => f,
        Err(e) => return e,
    };
    if !fields.is_empty() {
        let mut q = quote!();
        for (field, ty) in fields {
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
