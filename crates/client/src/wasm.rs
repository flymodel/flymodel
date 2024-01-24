#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(not(feature = "wasm"))]
pub fn log(s: &str) {
    println!("{}", s);
}
