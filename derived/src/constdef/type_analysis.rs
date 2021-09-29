use super::types::{DefExpr, CONSTDEF};
use quote::ToTokens;
use syn::TypePath;

pub(crate) fn analyze_type_path_shallow(t: &TypePath) -> Option<DefExpr> {
    if t.path.segments.len() == 1 {
        // one segment, so we may have this
        if t.path.segments[0].arguments.is_empty() {
            // no path args, so this is definitely something we may have
            let type_str = t.clone().into_token_stream().to_string();
            CONSTDEF.get(type_str.as_str()).cloned()
        } else {
            None
        }
    } else {
        None
    }
}
