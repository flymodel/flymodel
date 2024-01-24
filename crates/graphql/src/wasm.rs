#[macro_export]
macro_rules! jsvalue {
    ($($typ: ident), + $(,)?) => {
        $(
            cfg_if::cfg_if! {
                if #[cfg(feature = "wasm")] {
                    impl From<$typ> for wasm_bindgen::JsValue {
                        fn from(value: $typ) -> Self {
                            serde_wasm_bindgen::to_value(&value).unwrap()
                        }
                    }
                }
            }
        )*
    };
}
