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

    (
        if #[cfg(feature  = $value: literal)] {
            $(#[$attr:meta])*
        } else if #[cfg(feature  = $value_it: literal)] {
            $(#[$attr_it:meta])*
        } for {
            $block: item
        }
    ) => {
        #[cfg(feature = $value)]
        $(#[$attr])*
        $block
        #[cfg(all(not(feature = $value), feature = $value_it))]
        $(#[$attr_it])*
        $block

        #[cfg(all(not(feature = $value), not(feature = $value_it)))]
        $block
    }
}
