extern crate proc_macro;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Type};

#[proc_macro_attribute]
pub fn message(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ret_type = syn::parse_macro_input!(attr as Type);

    let i = input.clone();
    let body = syn::parse_macro_input!(i as DeriveInput);
    let impl_type = body.ident;

    let impl_message_for_type = impl_message(&impl_type, &ret_type);
    let input: TokenStream = input.into();

    let gen = quote! {
        #input
        #impl_message_for_type
    };
    gen.into()
}

fn impl_message(mname: &Ident, ret_name: &Type) -> TokenStream {
    quote! {
        impl Message for #mname {
            type Result = #ret_name;
        }
    }
}
