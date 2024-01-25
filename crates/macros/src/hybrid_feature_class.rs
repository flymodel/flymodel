use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input};

struct Targets {
    targets: Vec<String>,
}

impl Targets {
    fn is_wasm(&self) -> bool {
        self.targets.contains(&"wasm".to_string())
    }

    fn is_python(&self) -> bool {
        self.targets.contains(&"python".to_string())
    }
}

const VALID_TARGETS: &[&str] = &["wasm", "python"];

impl Parse for Targets {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut targets = vec![];

        while !input.is_empty() {
            let target = input.parse::<syn::LitStr>()?.value().to_lowercase();
            if !VALID_TARGETS.contains(&target.as_str()) {
                return Err(syn::Error::new_spanned(
                    target,
                    format!(
                        "Invalid target, valid targets are: {:?}",
                        VALID_TARGETS.join(",")
                    ),
                ));
            }
            targets.push(target);
            // consume the comma
            match input.parse::<syn::Token![,]>() {
                Ok(_) => {}
                Err(_) => break, // no more commas, stop parsing
            }
        }

        Ok(Self { targets })
    }
}

pub fn hybrid_feature_class_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let targets = parse_macro_input!(args as Targets);

    let wasm_variant = if targets.is_wasm() {
        quote! {
            #[cfg(feature = "wasm")]
            #[wasm_bindgen::prelude::wasm_bindgen]
            #item
        }
    } else {
        quote!()
    };

    let python_variant = if targets.is_python() {
        quote! {
            #[cfg(feature = "python")]
            #[pyo3::prelude::pyclass]
            #item
        }
    } else {
        quote!()
    };

    let fallback = match (targets.is_python(), targets.is_wasm()) {
        (true, true) => {
            quote! {
                #[cfg(all(not(feature = "python"), not(feature = "wasm")))]
                #item
            }
        }
        (true, false) => {
            quote! {
                #[cfg(not(feature = "python"))]
                #item
            }
        }
        (false, true) => {
            quote! {
                #[cfg(not(feature = "wasm"))]
                #item
            }
        }
        (false, false) => {
            quote! {
                #item
            }
        }
    };

    quote! {
        #wasm_variant
        #python_variant
        #fallback
    }
    .into()
}
