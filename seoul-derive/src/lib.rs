mod isomorphism;
use isomorphism::*;

use proc_macro2::TokenStream as TokenStream;
use quote::quote;
use syn::{self, DeriveInput, Data, Fields, Ident, Expr, spanned::Spanned, Result, Error};

/// Derive macro "Isomorphism"
/// 
/// * Implement traits:
///   * Isomorphism(methods of `title(&self)` and `list()`),
///   * Into<T> for &Self and Self
///   * From<T> for Self
/// 
/// * Only for Enum type.
///
/// # Fallbacks
///   * When Into type's value is not given at variant level attribute, default value will be used.
///   * If title is not given ant varaint level, **the variant's name (Ident)** will be used as titile.
///   * If list is not given at the top level attribute, list of each variant's default format will be returned.
/// 
/// 
/// # Ex.
/// ```
/// #[derive(Default, Isomorphism)]
/// #[isomorphism(u8, list=[A, B])]
/// pub enum ABC {
///   #[default] #[title("a")] A,
///   #[into(10)] #[title("b")] B,
///   #[into(100)] C
/// }
/// 
/// // Into
/// assert_eq!(Into::<u8>::into(ABC::A), 0);
/// assert_eq!(Into::<u8>::into(&ABC::B), 10);
/// // From
/// assert_eq!(Into::<ABC>::into(0u8), ABC::A);
/// assert_eq!(Into::<ABC>::into(100u8), ABC::C);
/// // List
/// assert_eq!(ABC::list(), vec![ABC::A, ABC::B]);
/// // Title
/// assert_eq!(ABC::A.title(), "A");
/// assert_eq!(ABC::C.title(), "C");
/// 
/// 
/// #[derive(Default, Isomorphism)]
/// #[isomorphism(into=[u8, i8])]
/// pub enum CD {
///   #[default] #[into([0, 1])] C,
///   #[into([0, -1])] D,
/// }
/// 
/// // Into
/// assert_eq!(Into::<u8>::into(CD::C), 0);
/// assert_eq!(Into::<i8>::into(CD::C), 1);
/// assert_eq!(Into::<u8>::into(CD::D), 0);
/// assert_eq!(Into::<i8>::into(CD::D), -1);
/// // From
/// assert_eq!(Into::<CD>::into(1i8), CD::C);
/// assert_eq!(Into::<CD>::into(-1i8), CD::D);
/// ```
/// 
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