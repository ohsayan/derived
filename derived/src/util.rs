use proc_macro::TokenStream;
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
