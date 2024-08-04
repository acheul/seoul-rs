use crate::*;

pub fn impl_tuplike_macro(ast: &DeriveInput) -> Result<TokenStream> {

  let name = &ast.ident;

  // get struct data
  let data = match &ast.data {
    Data::Struct(data) => data,
    _ => return Err(Error::new(ast.span(), "Only for Struct type"))
  };

  let mut tuple_token = TokenStream::new(); // tuple type
  let mut ref_tuple_token = TokenStream::new(); // ref tuple type
  let mut build_token: TokenStream = TokenStream::new();
  let mut is_tuple_struct = false;

  let mut into_token: TokenStream = TokenStream::new();
  let mut ref_into_token: TokenStream = TokenStream::new();
  
  data.fields.iter().enumerate().for_each(|(index, field)| {
    let ty = field.ty.to_token_stream();
    tuple_token.extend(quote! { #ty , });
    ref_tuple_token.extend(quote! { &'tpl #ty , });

    let index = syn::Index::from(index);
    if let Some(name) = &field.ident {
      build_token.extend(quote! { #name : value . #index ,   });
      into_token.extend(quote! { self . #name , });
      ref_into_token.extend(quote! { & self . #name , });
    } else {
      if is_tuple_struct != true { is_tuple_struct = true; }
      build_token.extend(quote! { value . #index , });
      into_token.extend(quote! { self . #index , });
      ref_into_token.extend(quote! { & self . #index , });
    }   
  });

  let tuple_token = quote! { (#tuple_token) };
  let ref_tuple_token = quote! { (#ref_tuple_token) };

  let build_token = if is_tuple_struct {
    quote! { Self( #build_token ) }
  } else {
    quote! { Self { #build_token } }
  };

  let into_token = quote! { ( #into_token ) };
  let ref_into_token = quote! { ( #ref_into_token ) };


  // parsing generics (add 'tpl lifetime for ref Into)
  let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

  let mut gen_clone = ast.generics.clone();
  let lt = syn::Lifetime::new("'tpl", Span::call_site());
  let ltp = syn::LifetimeParam::new(lt);
  gen_clone.params.push(syn::GenericParam::from(ltp));
  
  let (ref_impl_generics, _, ref_where_clause) = gen_clone.split_for_impl();


  // finalize
  let gen = quote! {

    impl #impl_generics Tuplike for #name #ty_generics #where_clause {
      type Tuple = #tuple_token;
    }

    impl #impl_generics From<#tuple_token> for #name #ty_generics #where_clause {
      fn from(value: #tuple_token) -> Self {
        #build_token
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
  };

  Ok(gen.into())
}