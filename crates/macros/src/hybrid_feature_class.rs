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

    #[darling(default)]
    ts: bool,
    #[darling(default)]
    into_ts: bool,
    #[darling(default)]
    from_ts: bool,

    #[darling(default)]
    rename_ts: bool,
    /// Whether to rename the `Serialize` method into camelCase
    #[darling(default)]
    rename_into_ts: bool,
    /// Whether to rename the `Deserialize` method into camelCase
    #[darling(default)]
    rename_from_ts: bool,

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

    fn is_ts(&self) -> bool {
        self.ts || self.into_ts || self.from_ts
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

    let wasm = if targets.is_wasm() && !targets.is_ts() {
        quote! {
            #[cfg_attr(
                feature = "wasm",
                wasm_bindgen::prelude::wasm_bindgen
            )]
        }
    } else {
        quote!()
    };

    let ts = if targets.is_ts() {
        let strm = if targets.ts {
            quote! {
                #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
                #[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
            }
        } else {
            match (targets.into_ts, targets.from_ts) {
                (true, true) => quote! {
                    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
                    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
                },
                (true, false) => quote! {
                    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
                    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
                },
                (false, true) => quote! {
                    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
                    #[cfg_attr(feature = "wasm", tsify(from_wasm_abi))]
                },
                _ => quote!(),
            }
        };

        let renames = if targets.rename_ts {
            quote! {
                #[cfg_attr(feature = "wasm", serde(rename_all = "camelCase"))]
            }
        } else {
            match (targets.rename_into_ts, targets.rename_from_ts) {
                (true, true) => quote! {
                    #[cfg_attr(feature = "wasm", serde(rename_all = "camelCase"))]
                },
                (true, false) => quote! {
                    #[cfg_attr(feature = "wasm", serde(rename_all(serialize = "camelCase")))]
                },
                (false, true) => quote! {
                    #[cfg_attr(feature = "wasm", serde(rename_all(deserialize = "camelCase")))]
                },
                _ => quote!(),
            }
        };

        quote! {
            #strm
            #renames
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
        #ts
        #item
    }
    .into()
}
