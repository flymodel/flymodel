use flymodel_macros::*;

use wasm_bindgen::prelude::*;

#[hybrid_feature_class("wasm", "python")]

struct SmokeTest {
    value: i32,
}

#[test]
fn test_it() {
    SmokeTest { value: 42 };
}
