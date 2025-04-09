extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(CannonicalStructure)]
pub fn derive_newtype(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let newtype_name = Ident::new(&format!("{}CannonicalStructure", name), name.span());

    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #newtype_name {}
        
        impl #newtype_name {
            fn new() -> Self {
                Self {}
            }
        }

        impl Structure for #newtype_name {}

        impl SetStructure for #newtype_name {
            type Set = #name;
        }

        impl MetaType for #name {
            type Structure = #newtype_name;
        
            fn structure() -> Self::Structure {
                #newtype_name::new()
            }
        }
    };

    TokenStream::from(expanded)
}