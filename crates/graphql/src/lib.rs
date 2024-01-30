#![allow(non_snake_case)]

#[cynic::schema("flymodel")]
pub mod schema {}

#[cfg(feature = "python")]
pub mod py;

pub mod enums;

pub mod fragments;
pub mod gql;
pub mod scalars;
pub mod wasm;

#[cfg(test)]
pub mod tests {}

#[macro_export]
macro_rules! new_for {
    (
        $(
            #[$($t: tt)*]
        )*
        $name:ident,
        $(
            $field:ident: $type:ty
        ), + $(,)?
    ) => {
        #[cfg(feature = "python")]
        #[pyo3::prelude::pymethods]
        impl $name {
            #[new]
            $(
                #[$($t)*]
            )*
            fn new($(
                $field: $type,
            )*) -> Self {
                Self {
                    $(
                        $field: $field.into(),
                    )*
                }
            }
        }
    };
}
