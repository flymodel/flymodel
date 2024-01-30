use std::fmt::Debug;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, FromMeta)]
struct Targets {
    #[darling(default)]
    wasm: bool,
    #[darling(default)]
    python: bool,

    py_getters: Option<bool>,
    py_setters: Option<bool>,
}

impl Targets {
    fn is_wasm(&self) -> bool {
        self.wasm
    }

    fn is_python(&self) -> bool {
        self.python
    }
}

pub fn hybrid_feature_class_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let targets = match Targets::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let setters = targets.py_setters.unwrap_or(false);
    let getters = targets.py_getters.unwrap_or(true);

    let py_extra = match (setters, getters) {
        (true, true) => quote! {
            get_all, set_all
        },
        (true, false) => quote! {
            set_all
        },
        (false, true) => quote! {
            get_all
        },
        (false, false) => quote!(),
    };

    let wasm = if targets.is_wasm() {
        quote! {
            #[cfg_attr(
                feature = "wasm",
                wasm_bindgen::prelude::wasm_bindgen
            )]
        }
    } else {
        quote!()
    };

    let python = if targets.is_python() {
        quote! {
            #[cfg_attr(
                feature = "python",
                pyo3::prelude::pyclass(#py_extra),
            )]
        }
    } else {
        quote!()
    };

    quote! {
        #wasm
        #python
        #item
    }
    .into()
}
