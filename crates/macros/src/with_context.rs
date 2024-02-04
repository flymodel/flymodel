use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Type};

pub(crate) fn with_context_impl(args: TokenStream) -> TokenStream {
    let input = parse_macro_input!(args as DeriveInput);
    let name = &input.ident;

    let data = match input.data {
        Data::Struct(ok) => ok,
        _ => panic!("must be a struct"),
    };

    let mut to_replicate = vec![];
    for arg in input.attrs {
        for seg in &arg.meta.path().segments {
            if seg.ident.to_string().as_str() == "context_needs" {
                let meta_list = arg.meta.require_list().expect("should have derives etc");
                let mut groups = vec![];
                let mut current_group = vec![];

                for att in meta_list.tokens.clone().into_iter() {
                    if att.to_string().as_str() == "," {
                        groups.append(&mut current_group);
                        current_group.clear();
                    } else {
                        current_group.push(att.to_token_stream())
                    }
                }
                groups.append(&mut current_group);
                let groups: Vec<proc_macro2::TokenStream> = groups
                    .into_iter()
                    .map(|it| it.into_iter().collect())
                    .collect();
                let groups: proc_macro2::TokenStream = groups.into_iter().collect();
                to_replicate.push(groups)
            }
        }
    }

    let to_replicate: proc_macro2::TokenStream = to_replicate.into_iter().collect();

    let mut normal = vec![];
    let mut vars = vec![];
    for field in data.fields {
        let mut found = false;
        for arg in &field.attrs {
            for seg in &arg.meta.path().segments {
                if seg.ident.to_string().as_str() == "context" {
                    found = true;
                    vars.push(field.clone());
                }
            }
        }
        if !found {
            normal.push(field);
            continue;
        }
    }

    if vars.is_empty() {
        panic!("expected mutants")
    }

    let ident_with_context =
        proc_macro2::TokenStream::from_str(&format!("{}WithContext", name.to_string()))
            .expect("ok");

    let fields: proc_macro2::TokenStream = normal
        .iter()
        .map(|it| {
            let toks = it.to_token_stream();
            quote! {
                #toks,
            }
        })
        .collect();

    let to_move: proc_macro2::TokenStream = normal
        .iter()
        .map(|it| {
            let id = it.ident.clone().expect("id");
            quote! {
                #id: self.#id.clone(),
            }
        })
        .collect();

    let as_new: proc_macro2::TokenStream = normal
        .iter()
        .map(|it| {
            let id = it.ident.clone().expect("id");
            let ty = it.ty.clone().to_token_stream();
            quote! {
                #id: #ty,
            }
        })
        .collect();

    let to_create: proc_macro2::TokenStream = normal
        .iter()
        .map(|it| {
            let id = it.ident.clone().expect("id");
            quote! {
                #id,
            }
        })
        .collect();

    let to_saturate: proc_macro2::TokenStream = vars
        .iter()
        .map(|it| {
            let toks = it.ident.clone().expect("ident");
            let ty = it.ty.clone().into_token_stream();
            quote! {
                #toks: #ty,
            }
        })
        .collect();

    let sat_no_ty: proc_macro2::TokenStream = vars
        .iter()
        .map(|it| {
            let name = it.ident.clone().expect("name");
            quote! {
                #name,
            }
        })
        .collect();

    quote! {
        #to_replicate
        pub struct #ident_with_context {
            #fields
        }

        #[cfg(feature = "python")]
        #[pyo3::prelude::pymethods]
        impl #ident_with_context {
            #[new]
            pub fn new( #as_new ) -> Self {
                Self { #to_create }
            }
        }

        #[cfg(not(feature = "python"))]
        impl #ident_with_context {
            pub fn new( #as_new ) -> Self {
                Self { #to_create }
            }
        }

        impl #ident_with_context {
            pub fn with_context(self, #to_saturate) -> #name {
                #name {
                    #to_move
                    #sat_no_ty
                }
            }
        }
    }
    .into()
}
