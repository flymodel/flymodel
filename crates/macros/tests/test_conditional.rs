use flymodel_macros::*;

#[hybrid_feature_class(wasm = true, python = true)]
struct SmokeTest {
    #[allow(dead_code)]
    value: i32,
}

#[test]
fn test_it() {
    SmokeTest { value: 42 };
}
