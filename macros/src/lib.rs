use proc_macro::TokenStream;
use quote::quote;

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
