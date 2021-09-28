use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type};

pub(crate) const ATTR_PHANTOM: &str = "phantom";

gen_typeset! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, str, bool, usize, isize, char, f32, f64
}

/// Returns the field names and their corresponding type from the AST (returning an error
/// if it isn't a struct)
pub fn get_struct_field_names(
    ast: &DeriveInput,
) -> Result<Vec<(&Ident, &Type, &Vec<Attribute>)>, TokenStream> {
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(
                syn::Error::new_spanned(ast, "this macro can only be used on structs")
                    .into_compile_error()
                    .into(),
            );
        }
    };
    if fields.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(fields
            .iter()
            .map(|field| {
                let fname = field.ident.as_ref().unwrap();
                (fname, &field.ty, &field.attrs)
            })
            .collect())
    }
}

pub(crate) fn single_instance_of_attr(
    attrs: &[Attribute],
    target: &str,
) -> Result<bool, TokenStream> {
    let mut span = None;
    let has_attr = attrs
        .iter()
        .filter(|attr| {
            span = Some((*attr).span());
            attr.path.is_ident(target)
        })
        .count();
    match has_attr {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(syn::Error::new(
            span.unwrap(),
            format!("Found duplicate attributes for `{}`", target),
        )
        .into_compile_error()
        .into()),
    }
}

/// Returns a const-ed (if required) func "header"
pub(crate) fn get_func_header(
    attrs: &[Attribute],
    target: &str,
) -> Result<quote::__private::TokenStream, TokenStream> {
    let r = if self::single_instance_of_attr(attrs, target)? {
        quote! {
            pub const fn
        }
    } else {
        quote! {
            pub fn
        }
    };
    Ok(r)
}
