use super::types::{DefExpr, CONSTDEF};
use ::quote::{quote, ToTokens};
use ::syn::{Type, TypePath, TypeTuple};

pub(crate) fn analyze_type_path(t: &TypePath) -> Option<DefExpr> {
    if t.path.segments.len() == 1 {
        // single, no need for extended analysis
        let fpath = t.path.segments[0].clone().into_token_stream().to_string();
        CONSTDEF.get(fpath.as_str()).cloned()
    } else {
        let r = t
            .path
            .segments
            .iter()
            .map(|seg| seg.ident.clone().into_token_stream().to_string())
            .collect();
        try_minimize_typepath(r)
            .ok()
            .and_then(|minimized| CONSTDEF.get(minimized.as_str()))
            .cloned()
    }
}

/// Attempt to minimize the type path
fn try_minimize_typepath(tpath: Vec<String>) -> Result<String, Vec<String>> {
    let mut path = tpath.iter();
    let mut ret = None;
    match path.next().map(|v| v.as_ref()) {
        Some(p) => match p {
            // attempt to evaluate paths like core::primitive::<ty> or std::primitive::<ty>
            "core" | "std" => match path.next() {
                Some(s) => match s.as_str() {
                    "primitive" => match path.next() {
                        Some(ty) => {
                            ret = Some(ty.to_owned());
                        }
                        None => {}
                    },
                    _ => {}
                },
                None => {}
            },
            _ => {}
        },
        None => {}
    }
    ret.ok_or(tpath)
}

pub fn recursive_process_tuple(tuple: &TypeTuple) -> Option<DefExpr> {
    let mut inner_decl = quote! {};
    for elem in tuple.elems.iter() {
        match elem {
            Type::Path(ref tpath) => {
                let ret = self::analyze_type_path(tpath)?.into_base_token();
                inner_decl = quote! {
                    #inner_decl
                    #ret,
                };
            }
            Type::Tuple(ref tuple) => {
                let ret = self::recursive_process_tuple(tuple)?.into_base_token();
                inner_decl = quote! {
                    #inner_decl
                    #ret,
                };
            }
            _ => return None,
        }
    }
    Some(DefExpr::CustomTuple(inner_decl.to_string()))
}
