#[macro_export]
macro_rules! jsvalue {
    ($($typ: ident), + $(,)?) => {
        $(
            cfg_if::cfg_if! {
                if #[cfg(feature = "wasm")] {
                    impl From<$typ> for wasm_bindgen::JsValue {
                        fn from(value: $typ) -> Self {
                            println!("call");
                            serde_wasm_bindgen::to_value(&value).unwrap()
                        }
                    }
                }
            }
        )*
    };
}

#[cfg(all(test, feature = "wasm"))]
mod test_exportable {

    use wasm_bindgen::JsValue;

    use crate::gql::create_bucket::CreateBucketVariables;

    jsvalue!(CreateBucketVariables);

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_to_js() {
        let _ = JsValue::from(CreateBucketVariables {
            name: "".into(),
            namespace_id: 1,
            region: None,
            role: crate::enums::Lifecycle::Test,
        });
    }
}
