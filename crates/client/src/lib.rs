cfg_if::cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[cfg(feature = "python")]
pub mod py;

#[cfg(feature = "tracing")]
pub mod trace;

pub mod client;

pub use client::{Client, Error};
