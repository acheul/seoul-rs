use crate::*;


pub fn impl_intowrap_macro(ast: &DeriveInput) -> Result<TokenStream> {

  let name = &ast.ident;
  let mut gen = TokenStream::new();

  // parsing generics
  let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

  match &ast.data {
    Data::Struct(data) => {
      if data.fields.len()==1 {
        let field = data.fields.iter().next().unwrap();
        let ty = field.ty.to_token_stream();
        let build = if let Some(name) = &field.ident {
          quote! { { #name : value } }
        } else {
          quote! { (value) }
        };

        gen.extend(quote! {
          impl #impl_generics From<#ty> for #name #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
              Self #build
            }
          }
        });
      } else {
        return Err(Error::new(ast.span(), "To implement `IntoWrap` to a struct, there must be just one field"))
      }
    },
    Data::Enum(data) => {
      for variant in data.variants.iter() {
        let variant_name = &variant.ident;
        let fields = &variant.fields;
        if fields.len()==1 {
          if let Fields::Unnamed(_) = fields {
    
            let field = fields.iter().next().unwrap();
            let ty = field.ty.to_token_stream();
    
            gen.extend(quote! {
              impl #impl_generics From<#ty> for #name #ty_generics #where_clause {
                fn from(value: #ty) -> Self {
                  Self :: #variant_name ( value )
                }
              }
            });
          }
        }
      }
    },
    Data::Union(_) => {
      return Err(Error::new(ast.span(), "Not for Union data type"))
    }
  }

  Ok(gen.into())
}