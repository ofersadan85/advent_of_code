use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro]
pub fn read_input(_: TokenStream) -> TokenStream {
    quote! {
        let file = std::path::PathBuf::from(file!());
        let day = file
            .with_extension("txt")
            .file_name()
            .expect("txt")
            .to_string_lossy()
            .into_owned();
        let year: u16 = file
            .parent()
            .expect("current directory")
            .parent()
            .expect("crate directory")
            .file_name()
            .and_then(|p| p.to_str())
            .map(|p| p.trim_start_matches('y'))
            .and_then(|p| p.parse().ok())
            .unwrap();
        let input = std::fs::read_to_string(format!("../inputs/{year}/{day}")).expect("Input file");
    }
    .into()
}

#[proc_macro_attribute]
pub fn aoc_tests(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemMod);
    if item.ident != "tests" {
        return syn::Error::new_spanned(
            item.ident,
            "aoc_tests can only be applied to a module named tests",
        )
        .to_compile_error()
        .into();
    }
    item.attrs.push(syn::parse_quote!(#[cfg(test)]));
    let use_statements = quote! {
        use super::*;
        use advent_of_code_macros::read_input;
        use test_log::test;
    };
    if let Some(content) = item.content.as_mut() {
        content.1.insert(0, syn::Item::Verbatim(use_statements));
    } else {
        item.content = Some((
            syn::token::Brace::default(),
            vec![syn::Item::Verbatim(use_statements)],
        ));
    }
    item.into_token_stream().into()
}
