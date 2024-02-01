use std::str::FromStr;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn hybrid_enum_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("HybridEnum can only be derived for enums"),
    };
    let mut py_new = vec![];
    let mut py_str = vec![];
    for variant in data.variants.iter() {
        let adj = variant.ident.to_string().to_lowercase();
        let adj = proc_macro2::TokenStream::from_str(&format!(r#""{adj}""#)).unwrap();
        let ident = &variant.ident;
        py_new.push(quote! { #adj => Ok(Self::#ident), });
        py_str.push(quote! { Self::#ident => #adj.to_string(),  });
    }

    let py_new = proc_macro2::TokenStream::from_iter(py_new.into_iter());
    let py_str = proc_macro2::TokenStream::from_iter(py_str.into_iter());
    let py_new = quote! {
        #[cfg(feature = "python")]
        #[pyo3::pymethods]
        impl #name {
            #[new]
            fn new(value: String) -> pyo3::PyResult<Self> {
                match value.as_str() {
                    #py_new
                    _ => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("{} is invalid", value))),
                }
            }

            fn __str__(&self) -> String {
                match self {
                    #py_str
                }
            }
        }
    };
    quote! {
        #py_new
    }
    .into()
}
