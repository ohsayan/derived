use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type};

/// Returns the field names and their corresponding type from the AST (returning an error
/// if it isn't a struct)
pub fn get_struct_field_names(ast: &DeriveInput) -> Result<Vec<(Ident, Type)>, TokenStream> {
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
                let ty = field.ty.clone();
                let fname = field.ident.as_ref().unwrap().clone();
                (fname, ty)
            })
            .collect())
    }
}

/// Returns a const-ed (if required) func "header"
pub fn get_func_header(
    attrs: &[Attribute],
    target: &str,
) -> Result<quote::__private::TokenStream, ()> {
    let has_attr = attrs
        .iter()
        .filter(|attr| attr.path.is_ident(target))
        .count();
    match has_attr {
        0 => Ok(quote! {
            pub fn
        }),
        1 => Ok(quote! {
            pub const fn
        }),
        _ => Err(())
    }
}
