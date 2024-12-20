use quote::quote;

pub(crate) fn enum_char_impls(input: syn::DeriveInput, display: bool) -> proc_macro::TokenStream {
    let ident = &input.ident;
    let mut to_value = vec![];
    let mut from_value = vec![];
    if let syn::Data::Enum(data) = &input.data {
        for variant in &data.variants {
            for attr in &variant.attrs {
                if attr.meta.path().is_ident("c") {
                    match attr.meta.require_name_value() {
                        Ok(meta) => {
                            let v_ident = &variant.ident;
                            let value = &meta.value;
                            to_value.push(quote! {
                                #ident::#v_ident => #value
                            });
                            from_value.push(quote! {
                                #value => Ok(Self::#v_ident)
                            });
                        }
                        Err(e) => {
                            return e.to_compile_error().into();
                        }
                    }
                }
            }
        }
    }
    let mut expanded = quote! {
        impl From<#ident> for char {
            fn from(v: #ident) -> Self {
                match v {
                    #(
                        #to_value,
                    )*
                }
            }
        }

        impl TryFrom<char> for #ident {
            type Error = &'static str;
            fn try_from(c: char) -> Result<Self, Self::Error> {
                match c {
                    #(
                        #from_value,
                    )*
                    _ => Err("Invalid char"),
                }
            }
        }
    };
    if display {
        expanded.extend(quote! {
            impl std::fmt::Display for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", char::from(*self))
                }
            }
        });
    }
    expanded.into()
}
