use hybrid_enum::hybrid_enum_impl;
use hybrid_feature_class::hybrid_feature_class_impl;
use proc_macro::TokenStream;
mod hybrid_enum;
mod hybrid_feature_class;

/// # Hybrid Feature Class
///
/// Offers common methods for deriving:
///     - python classes
///     - wasm classes
///     - typescript interfaces
///
/// Good to remember:
///     - This is only used for common serialization methods
///     - rename_into_ts / rename_from_ts MUST not conflict with the De / Ser methods that the server expects
#[proc_macro_attribute]
pub fn hybrid_feature_class(args: TokenStream, item: TokenStream) -> TokenStream {
    hybrid_feature_class_impl(args, item)
}

#[proc_macro_derive(HybridEnum)]
pub fn hybrid_enum(args: TokenStream) -> TokenStream {
    hybrid_enum_impl(args)
}
