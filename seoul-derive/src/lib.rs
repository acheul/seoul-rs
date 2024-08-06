mod isomorphism;
use isomorphism::*;

mod tuplike;
use tuplike::*;

mod reflica;
use reflica::*;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{self, DeriveInput, Data, Fields, Ident, Expr, spanned::Spanned, Result, Error};


#[proc_macro_derive(Reflica, attributes(reflica))]
pub fn reflica_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();

  impl_reflica_macro(&ast)
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}


#[proc_macro_derive(Tuplike)]
pub fn tuplike_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();

  impl_tuplike_macro(&ast)
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}


#[proc_macro_derive(Isomorphism, attributes(isomorphism, into, title))]
pub fn isomorphism_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse(input).unwrap();

  impl_isomorphism_macro(&ast)
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}


fn fields_default_format(fields: &syn::Fields) -> Result<TokenStream> {

  let quoted = match fields {
    Fields::Named(fields) => {
      let mut quoted = TokenStream::new();
      let len = fields.named.len();

      for (i, field) in fields.named.iter().enumerate() {
        let name = field.ident.as_ref().unwrap();
        let x = if i+1==len { quote! { #name: Default::default() } } else { quote! { #name: Default::default(), } };
        quoted.extend(x);
      }

      quote! {
        {#quoted}
      }
    },
    Fields::Unnamed(fields) => {
      let mut quoted = TokenStream::new();
      let len = fields.unnamed.len();

      for (i, _field) in fields.unnamed.iter().enumerate() {
        let x = if i+1==len { quote! { Default::default() } } else { quote! { Default::default(), } };
        quoted.extend(x);
      }

      quote! {
        (#quoted)
      }
    },
    Fields::Unit => TokenStream::new(),
  };

  Ok(quoted.into())
}


fn variant_matching_format(ty_name: &Ident, variant: &syn::Variant)-> Result<TokenStream> {

  let variant_name = &variant.ident;

  let gen = match &variant.fields {
    Fields::Named(_) => quote! { #ty_name::#variant_name {..} },
    Fields::Unnamed(_) => quote! { #ty_name::#variant_name(..) },
    Fields::Unit => quote! { #ty_name::#variant_name }
  };

  Ok(gen.into())
}


fn variant_default_format(ty_name: &Ident, variant: &syn::Variant) -> Result<TokenStream> {
  
  let variant_name = &variant.ident;
  let quoted_fields = fields_default_format(&variant.fields)?;

  let gen = quote! {
    #ty_name::#variant_name #quoted_fields
  };
  Ok(gen.into())
}