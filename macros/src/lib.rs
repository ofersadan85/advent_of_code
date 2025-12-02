use quote::{quote, ToTokens};
use syn::parse_macro_input;

mod impls;

fn read_input_fn() -> proc_macro2::TokenStream {
    quote! {
        fn read_input() -> String {
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
            std::fs::read_to_string(format!("../inputs/{year}/{day}")).expect("Input file")
        }
    }
}

#[proc_macro_attribute]
pub fn aoc_tests(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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
        use test_log::test;
    };
    if let Some(content) = item.content.as_mut() {
        content.1.insert(0, syn::Item::Verbatim(use_statements));
        content.1.push(syn::Item::Verbatim(read_input_fn()));
    } else {
        item.content = Some((
            syn::token::Brace::default(),
            vec![syn::Item::Verbatim(use_statements)],
        ));
    }
    item.into_token_stream().into()
}

/// A procedural macro to include all day modules up to a specified last day.
/// If no last day is specified, defaults to 25.
/// 
/// # Panics
/// 
/// Panics if the provided last day is not a valid integer
#[proc_macro]
pub fn all_the_days(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let last: u8 = if input.is_empty() {
        25
    } else {
        syn::parse_macro_input!(input as syn::LitInt)
            .base10_parse()
            .expect("valid integer")
    };
    if last <= 10 {
        quote! {
            use seq_macro::seq;
            seq!(N in 1..=#last {
                pub mod day0~N;
            });
        }
    } else {
        quote! {
            use seq_macro::seq;
            seq!(N in 1..10 {
                pub mod day0~N;
            });
            seq!(N in 10..=#last {
                pub mod day~N;
            });
        }
    }
    .into()
}

#[proc_macro_derive(CharEnum, attributes(c))]
pub fn derive_char_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    impls::enum_char_impls(&input, false)
}

#[proc_macro_derive(CharEnumDisplay, attributes(c))]
pub fn derive_char_enum_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    impls::enum_char_impls(&input, true)
}

/// A procedural macro to define an enum where each variant is associated with a `char` value.
/// 
/// # Panics
/// 
/// Panics if any variant does not have a `char` discriminant.
#[proc_macro_attribute]
pub fn char_enum(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemEnum);
    let derive_variant = if attr
        .into_iter()
        .next()
        .is_some_and(|a| a.to_string() == "display")
    {
        quote! {advent_of_code_macros::CharEnumDisplay}
    } else {
        quote! {advent_of_code_macros::CharEnum}
    };
    if item.attrs.is_empty() {
        item.attrs
            .push(syn::parse_quote!(#[derive(Debug, PartialEq, Eq, Clone, Copy, #derive_variant)]));
    } else {
        for attr in &mut item.attrs {
            if attr.meta.path().is_ident("derive") {
                if let syn::Meta::List(ref mut list) = attr.meta {
                    list.tokens
                        .extend(quote! {, Debug, PartialEq, Eq, Clone, Copy, #derive_variant});
                }
            }
        }
    }
    for v in &mut item.variants {
        if v.discriminant.is_none() {
            return syn::Error::new_spanned(
                v,
                "#[char_enum] variants must have a `char` discriminant",
            )
            .to_compile_error()
            .into();
        }
        let value = v.discriminant.clone().expect("checked above").1;
        v.discriminant = None;
        v.attrs.push(syn::parse_quote! {#[c = #value]});
    }
    item.to_token_stream().into()
}
