use hybrid_feature_class::hybrid_feature_class_impl;
use proc_macro::TokenStream;

mod hybrid_feature_class;

#[proc_macro_attribute]
pub fn hybrid_feature_class(args: TokenStream, item: TokenStream) -> TokenStream {
    hybrid_feature_class_impl(args, item)
}
