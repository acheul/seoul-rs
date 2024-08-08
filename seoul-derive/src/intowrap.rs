use crate::*;

pub fn impl_intowrap_macro(ast: &DeriveInput) -> Result<TokenStream> {

  let name = &ast.ident;
  let mut gen = TokenStream::new();

  let Data::Enum(data) = &ast.data else {
    return Err(Error::new(ast.span(), "Only for Enum data type"))
  };

  // parsing generics
  let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

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

  Ok(gen.into())
}