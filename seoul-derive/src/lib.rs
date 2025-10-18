#![doc = include_str!("../../README.md")]

mod isomorphism;
mod reflica;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{self, spanned::Spanned, Data, DeriveInput, Error, Expr, Fields, Ident, Result};


/// # `Isomorphism`
#[proc_macro_derive(Isomorphism, attributes(isomorphism, into_field, from_variant, name, into, restore))]
pub fn isomorphsim_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    isomorphism::impl_isomorphism_macro(&ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}


/// # `Reflica`
#[proc_macro_derive(Reflica, attributes(reflica))]
pub fn reflica_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    reflica::impl_reflica_macro(&ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}