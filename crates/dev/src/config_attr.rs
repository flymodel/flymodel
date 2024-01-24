#[macro_export]
macro_rules! config_attr {
    (
        if #[cfg(feature  = $value: literal)] {
            $(#[$attr:meta])*
        } for {
            $block: item
        }
    ) => {
            #[cfg(feature = $value)]
            $(#[$attr])*
            $block
            #[cfg(not(feature = $value))]
            $block
    };
}
