use crate::*;

pub fn impl_tuplike_macro(ast: &DeriveInput) -> Result<TokenStream> {

  let name = &ast.ident;
  let mut gen = TokenStream::new();

  // parsing generics
  let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

  // add 'tpl lifetime for ref Into to the generics
  let mut gen_clone = ast.generics.clone();
  let lt = syn::Lifetime::new("'tpl", Span::call_site());
  let ltp = syn::LifetimeParam::new(lt);
  gen_clone.params.push(syn::GenericParam::from(ltp));

  let (ref_impl_generics, _, ref_where_clause) = gen_clone.split_for_impl();


  // impl Tuplike trait
  gen.extend(quote! {

    impl #impl_generics Tuplike for #name #ty_generics #where_clause {
      
    }
  });

  match &ast.data {
    Data::Struct(data) => {

      let Some((
        tuple_token,
        ref_tuple_token,
        build_token,
        into_token,
        ref_into_token
      )) = get_tokens(&data.fields) else {
        return Err(Error::new(ast.span(), "Not for 0 fields data"))
      };

      let tuple_args = expanded_tuple_argument(&data.fields);

      gen.extend(quote! {

        impl #impl_generics From<#tuple_token> for #name #ty_generics #where_clause {
          fn from(#tuple_args: #tuple_token) -> Self {
            Self #build_token
          }
        }

        impl #impl_generics Into<#tuple_token> for #name #ty_generics #where_clause {
          fn into(self) -> #tuple_token {
            #into_token
          }
        }

        impl #ref_impl_generics Into<#ref_tuple_token> for &'tpl #name #ty_generics #ref_where_clause {
          fn into(self) -> #ref_tuple_token {
            #ref_into_token
          }
        }
      });
    },

    Data::Enum(data) => {

      for variant in data.variants.iter() {
        let variant_name = &variant.ident;

        if let Some((
          tuple_token,
          _ref_tuple_token,
          build_token,
          _into_token,
          _ref_into_token
        )) = get_tokens(&variant.fields) {

          let tuple_args = expanded_tuple_argument(&variant.fields);
  
          gen.extend(quote! {
    
            impl #impl_generics From<#tuple_token> for #name #ty_generics #where_clause {
              fn from(#tuple_args: #tuple_token) -> Self {
                Self :: #variant_name #build_token
              }
            }
          });
        }
      }     
    },

    Data::Union(_) => {
      return Err(Error::new(ast.span(), "Not for Union data type"))
    }
  }

  Ok(gen.into())
}


fn expanded_tuple_argument(fields: &Fields) -> TokenStream {

  let mut token = TokenStream::new();
  for i in 0..fields.len() {
    let f = Ident::new(&format!("f{}", i), fields.span());
    token.extend(quote! { #f , });
  }
  quote! { (#token) }
}


/// for Struct fields
/// * return (
///   tuple-token,
///   ref-tuple-token,
///   build-token,
///   into-token,
///   ref-into-token,
/// )
/// * return None if length==0
fn get_tokens(fields: &Fields) -> Option<(
  TokenStream,
  TokenStream, TokenStream,
  TokenStream, TokenStream,
)> {

  let mut tuple_token: TokenStream = TokenStream::new();
  let mut ref_tuple_token: TokenStream = TokenStream::new();
  let mut build_token: TokenStream = TokenStream::new();
  let mut into_token: TokenStream = TokenStream::new();
  let mut ref_into_token: TokenStream = TokenStream::new();

  let len = fields.len();
  if len==0 {
    return None
  }
  let is_named = if let Fields::Named(_) = fields { true } else { false };

  for (i, field) in fields.iter().enumerate() {

    let ty = field.ty.to_token_stream();
    tuple_token.extend(quote! { #ty, });
    ref_tuple_token.extend(quote! { &'tpl #ty, });
    
    let index = syn::Index::from(i);
    let f = Ident::new(&format!("f{}", i), fields.span());

    if let Some(name) = &field.ident {
      build_token.extend(quote! { #name : #f, });
      into_token.extend(quote! { self . #name , });
      ref_into_token.extend(quote! { &self . #name , });
    } else {
      build_token.extend(quote! { #f , });
      into_token.extend(quote! { self . #index , });
      ref_into_token.extend(quote! { &self . #index , });
    }
  }

  build_token = if is_named { quote! { { #build_token } } } else { quote! { (#build_token) } };
  tuple_token = quote! { (#tuple_token) };
  ref_tuple_token = quote! { (#ref_tuple_token) };

  into_token = quote! { (#into_token) };
  ref_into_token = quote! { (#ref_into_token) };

  Some((
    tuple_token,
    ref_tuple_token,
    build_token,
    into_token,
    ref_into_token
  ))
}