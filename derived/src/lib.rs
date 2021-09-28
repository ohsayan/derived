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
#[macro_use]
mod macros;
mod ctor;
mod gtor;
mod stor;
mod util;

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
    ctor::derive_ctor(input)
}

#[proc_macro_derive(Gtor, attributes(const_gtor, gtor_copy))]
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
    gtor::derive_gtor(input)
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
    stor::derive_stor(input)
}
