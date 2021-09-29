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
//! - [`Constdef`]: To generate constant, compile-time default implementations
//! - Full lifetimes, generics and `where` clause support
//! - Use the `gtor` attribute to get either immutable or mutable or both references (see example below)
//! - Skip generation of setters or getters with the `#[stor_skip]` or `#[gtor_skip]` attributes for
//!   specific fields
//! - Make ctors and gtors `const` with the `#[ctor_const]` and `#[gtor_const]` attributes
//! - Skip ctors, gtors and stors for `PhantomData` fields with the `#[phantom]` attribute
//!

use ::proc_macro::TokenStream;
#[macro_use]
mod macros;
mod constdef;
mod ctor;
mod gtor;
mod stor;
mod util;

#[proc_macro_derive(Ctor, attributes(ctor_const, phantom))]
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
/// # Attributes
///
/// The following attributes are available:
/// - `#[ctor_const]`: Will make your ctors constant
/// - `#[phantom]`: Will skip the specified [`PhantomData`](core::marker::PhantomData) field(s) in
/// the constructor, automatically adding `PhantomData` in the requisite positions
///
/// ## Constant constructors
///
/// To make your constructors `const`, simply add the `#[ctor_const]` attribute to the top
/// of your struct.
///
/// ### Example
///
/// ```
/// use derived::Ctor;
///
/// #[derive(Ctor)]
/// #[ctor_const]
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

#[proc_macro_derive(Gtor, attributes(gtor_const, gtor_copy, gtor_skip, phantom, gtor))]
/// # Gtor: Get the getters derived
///
/// Gtor takes the fields in order and generates getters for each field. For example,
/// if you have fields named `userid` and `name`, then the getters generated will be
/// `get_userid` and `get_name`, returning references to the appropriate types. In other
/// words, `get_*` named methods will be derived per your fields.
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
/// # Attributes
///
/// The following attributes are available:
/// - `#[gtor_const]`: Will make your gtors constant
/// - `#[gtor_skip]`: Will skip generation of getters for specific fields
/// - `#[gtor_copy]`: Makes the getter return a copy of the value, assuming that the type is [`Copy`]
/// - `#[phantom]`: Marks the field as a [`PhantomData`](core::marker::PhantomData) field, hence
///     skipping getters, setters and ctors for the field
/// - `#[gtor(...)]`: See [this example](#the-gtor-attribute)
///
/// ## The `gtor` attribute
///
/// Simply add the gtor attribute like this: `#[gtor(get, get_mut)]` on the top of your struct to
/// get mutable and immutable reference methods to the fields in your struct.
///
/// ### Example
///
/// ```
/// use derived::{Ctor, Gtor};
/// #[derive(Ctor, Gtor)]
/// #[gtor(get, get_mut)]
/// pub struct Mutable {
///     x_axis: u8,
///     y_axis: u8,
/// }
///
/// #[test]
/// fn test_get_and_get_mut() {
///     let mut m = Mutable::new(0, 0);
///     // move x by 1 unit
///     *m.get_x_axis_mut() = 1;
///     // move y by 2 units
///     *m.get_y_axis_mut() = 2;
///     assert_eq!(m.get_x_axis(), 1);
///     assert_eq!(m.get_y_axis(), 2);
/// }
/// ```
///
/// # Important notes
///
/// ## References
/// If any of the fields within the struct are primitive types that do not require large copies,
/// then the value is returned directly instead of a reference to it:
/// ```text
/// u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, str, bool, usize, isize, char, f32, f64
/// ```
///
/// ## Doc-comments
///
/// The [`Gtor`] macro will automatically add a doc comment of the form:
/// ```text
/// Returns the value for the `<struct_field>` field in struct [`<struct_name>`]
/// ```
///
pub fn derive_gtor(input: TokenStream) -> TokenStream {
    gtor::derive_gtor(input)
}

#[proc_macro_derive(Stor, attributes(stor_skip, phantom))]
/// # Stor: Get the setters derived
///
/// Stor takes the fields in order and generates setters for each field. For example,
/// if you have fields named `userid` and `name`, then the setters generated will be
/// `set_userid` and `set_name`, accepting values for the appropriate types. In other
/// words, `set_*` named methods will be derived per your fields.
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
///
/// # Attributes
///
/// The following attributes are available:
/// - `#[phantom]`: Skips the stor for the specified field(s), assuming they are
/// [`PhantomData`](core::marker::PhantomData) fields. This has the same effect as `#[stor_skip]`
/// but it makes it easier to use with the other macros, avoiding the need to write skips for phantom
/// fields specifically
/// - `#[stor_skip]`: Skips the stor for the specified field(s)
///
/// ## Doc-comments
///
/// The [`Stor`] macro will automatically add a doc comment of the form:
/// ```text
/// Sets the value for the `<struct_field>` field in struct [`<struct_name>`]
/// ```
///
pub fn derive_stor(input: TokenStream) -> TokenStream {
    stor::derive_stor(input)
}

#[proc_macro_derive(Constdef)]
/// # `Constdef`: Constant, compile-time default implementations
///
/// Overcome the limits of the default trait to get constant, compile-time default implementations.
///
/// ## Why, and how?
/// Implementations of the [`Default`](core::default::Default) trait cannot unfortunately be called
/// in `const` contexts due to the [current limitations with traits per RFC 911](https://rust-lang.github.io/rfcs/0911-const-fn.html#detailed-design).
/// To overcome this limitation, this crate _hacks around_ the problem by evaluating types at
/// compile time and substituting requisite values. A `const fn default()` is implemented for the struct,
/// along with the [`Default`] trait, enabling you to use it other contexts that need you to use
/// default values, along with `const` contexts.
///
/// ## Example
///
/// ```
/// use derived::Constdef;
///
/// #[derive(Constdef)]
/// struct MyDefault {
///     a: u8,
///     b: bool,
///     c: char,
///     d: f32,
///     e: (),
///     f: u128,
///     array: [f32; 16], // arrays too!
///     char_array: [char; 48],
///     // even tuples!
///     tuple: (u8, u16),
///     nested_tuple: ((u8, u8), u16),
/// }
///
/// const DEF: MyDefault = MyDefault::default();
/// assert_eq!(DEF.a, 0);
/// assert_eq!(DEF.b, false);
/// assert_eq!(DEF.c, '\0');
/// assert_eq!(DEF.d, 0.0);
/// assert_eq!(DEF.e, ());
/// assert_eq!(DEF.f, 0);
/// assert_eq!(DEF.array, [0.0; 16]);
/// assert_eq!(DEF.char_array, ['\0'; 48]);
/// assert_eq!(DEF.tuple, (0, 0));
/// assert_eq!(DEF.nested_tuple, ((0, 0), 0));
///
/// // you can also use it with methods that use `Default` because the trait
/// // is implemented too
/// let mut x: Option<MyDefault> = None;
/// let y = x.unwrap_or_default();
/// assert_eq!(y.c, '\0');
/// ```
///
/// ## Supported types
///
/// - The following primitive types are supported:
///     ```text
///     u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, str, bool, usize, isize, char, f32, f64, ()
///     ```
/// - All arrays of the above types are supported
/// - All tuples and nested tuples of the above types are supported
/// - Nested arrays are not yet supported, but is being worked on
pub fn derive_constdef(input: TokenStream) -> TokenStream {
    constdef::derive(input)
}
