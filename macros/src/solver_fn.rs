use quote::quote;
use std::{
    collections::HashMap,
    path::{self, PathBuf},
};
use syn::spanned::Spanned;

const ALLOWED_ATTRIBUTES: [&str; 3] = ["file", "input", "expected"];

pub struct AttrPairs {
    pub(crate) pairs: HashMap<String, syn::MetaNameValue>,
    pub(crate) span: proc_macro2::Span,
}

impl syn::parse::Parse for AttrPairs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut pairs = HashMap::new();
        while !input.is_empty() {
            let pair: syn::MetaNameValue = input.parse()?;
            let Some(ident) = pair.path.get_ident() else {
                return Err(syn::Error::new(
                    pair.path.span(),
                    "Expected a single identifier for the attribute key",
                ));
            };
            pairs.insert(ident.to_string(), pair);
            if input.peek(syn::Token![,]) {
                let _comma: syn::Token![,] = input.parse()?; // Consume the comma
            }
        }
        Ok(Self {
            pairs,
            span: input.span(),
        })
    }
}

fn str_value_from_meta_name_value(meta: &syn::MetaNameValue) -> Option<String> {
    if let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit_str),
        ..
    }) = &meta.value
    {
        Some(lit_str.value())
    } else {
        None
    }
}

pub fn impl_solver(
    f: &syn::ItemFn,
    mut attrs: AttrPairs,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    if attrs.pairs.is_empty() {
        return Err(syn::Error::new(
            attrs.span,
            "Expected attributes: 'expected' and either 'file' or 'input'",
        ));
    }

    // Parse the attribute arguments
    let attr_span = attrs.span;
    let f_name = &f.sig.ident;
    let expected = match attrs.pairs.remove("expected") {
        Some(meta) => meta.value,
        _ => {
            return Err(syn::Error::new(
                attr_span,
                "Expected attribute pair 'expected = <expression>'",
            ));
        }
    };
    let mut needs_ref = false;
    let input_expr = match (attrs.pairs.remove("file"), attrs.pairs.remove("input")) {
        (Some(file_meta), None) => {
            let file_literal = str_value_from_meta_name_value(&file_meta).ok_or_else(|| {
                syn::Error::new_spanned(
                    &file_meta,
                    "Expected a string literal for 'file' attribute",
                )
            })?;
            let path = match path::absolute(PathBuf::from(file_literal)) {
                Ok(p) => p,
                Err(_) => {
                    return Err(syn::Error::new_spanned(
                        file_meta.value,
                        "Could not resolve absolute path",
                    ));
                }
            }
            .display()
            .to_string();
            needs_ref = true;
            quote! {
                ::std::fs::read_to_string(#path).expect(&format!("Failed to read file: {}", #path))
            }
        }
        (None, Some(input_meta)) => {
            let value = input_meta.value;
            quote! {#value}
        }
        _ => {
            return Err(syn::Error::new(
                attr_span,
                "You must provide either 'file' or 'input' attributes, but not both",
            ));
        }
    };
    let f_call = if needs_ref {
        quote! {#f_name(&input)}
    } else {
        quote! {#f_name(input)}
    };
    let mut prefix = attrs
        .pairs
        .remove("prefix")
        .and_then(|m| str_value_from_meta_name_value(&m))
        .unwrap_or_default();
    if !prefix.ends_with('_') && !prefix.is_empty() {
        prefix.push('_');
    }
    let mut suffix = attrs
        .pairs
        .remove("suffix")
        .and_then(|m| str_value_from_meta_name_value(&m))
        .unwrap_or_default();
    if !suffix.starts_with('_') && !suffix.is_empty() {
        suffix.insert(0, '_');
    }
    let test_name = syn::Ident::new(&format!("test_{prefix}{f_name}{suffix}"), f_name.span());
    Ok(quote! {
        #f

        #[cfg(test)]
        #[test]
        fn #test_name() {
            let input = #input_expr;
            let result = #f_call;
            assert_eq!(result, #expected);
        }
    })
}
