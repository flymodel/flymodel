cfg_if::cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[cfg(feature = "python")]
pub mod py;

#[cfg(feature = "tracing")]
pub mod trace;

pub mod artifacts;

pub mod client;

pub use client::{Client, Error};

pub mod maybe;

pub mod experiment;

#[cfg(all(feature = "tracing", feature = "wasm"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    crate::trace::init_subscriber();
    Ok(())
}
