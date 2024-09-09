use crate::*;

pub fn impl_reflica_macro(ast: &DeriveInput) -> Result<TokenStream> {

  let mut gen = TokenStream::new();
  let name = &ast.ident;

  // attributes (collect derive idents and prefix)
  let mut prefix: Option<syn::LitStr> = None;

  let derive_quote = match ast.attrs.iter().find(|x| x.path().is_ident("reflica")) {
    Some(attr) => {
      let mut quote = TokenStream::new();
      let mut init = false;

      attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("prefix") {
          let arg: syn::LitStr = meta.value()?.parse()?;
          prefix.replace(arg);

        } else if let Some(ident) = meta.path.get_ident() {
          if init {
            quote.extend(quote! { , #ident })
          } else {
            init = true;
            quote.extend(quote! { #ident })
          }
        }
        Ok(())
      })?;

      quote! { #[derive(#quote)] }
    },
    None => quote! { }
  };

  // ref name
  let ref_name = if let Some(prefix) = prefix {
    format!("{}{}", prefix.value(), name)
  } else {
    format!("Ref{}", name)
  };
  let ref_name = Ident::new(&ref_name, ast.span());

  // parsing generics (add 'a lifetime for ref type)
  let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

  let mut gen_clone = ast.generics.clone();
  let lt = syn::Lifetime::new("'a", Span::call_site());
  let ltp = syn::LifetimeParam::new(lt);
  gen_clone.params.push(syn::GenericParam::from(ltp));
  
  let (ref_impl_generics, ref_ty_generics, ref_where_clause) = gen_clone.split_for_impl();
  let vis = ast.vis.to_token_stream();

  // for each data type 
  match &ast.data {
    // struct
    Data::Struct(data) => {
      let (is_named, mut ref_fields, match_fields) = get_ref_fields(&data.fields);
      if !is_named {
        ref_fields.extend(quote! { ; });
      }
      
      gen.extend(quote! {

        // trait Reflica
        impl #impl_generics Reflica for #name #ty_generics #where_clause { }

        // declare ref type
        #derive_quote
        #vis struct #ref_name #ref_impl_generics #where_clause
        #ref_fields
      
        // Into
        impl #ref_impl_generics Into<#ref_name #ref_ty_generics> for &'a #name #ty_generics #ref_where_clause {
          fn into(self) -> #ref_name #ref_ty_generics {
            let #name #match_fields = self;
            #ref_name #match_fields
          }
        }
      });
    },

    // enum
    Data::Enum(data) => {

      let mut ref_variants = TokenStream::new();
      let mut match_variants = TokenStream::new();
        
      for variant in data.variants.iter() {
        let ident = &variant.ident;
        let (_, ref_fields, match_fields) = get_ref_fields(&variant.fields);
        
        ref_variants.extend(quote! {
          #ident #ref_fields ,
        });

        match_variants.extend(quote! {
          #name :: #ident #match_fields => #ref_name :: #ident #match_fields ,
        });
      }

      gen.extend(quote! {

        // trait Reflica
        impl #impl_generics Reflica for #name #ty_generics #where_clause { }

        // declare ref type
        #derive_quote
        #vis enum #ref_name #ref_impl_generics #where_clause
        { #ref_variants }
        
        // Into
        impl #ref_impl_generics Into<#ref_name #ref_ty_generics> for &'a #name #ty_generics #ref_where_clause {
          fn into(self) -> #ref_name #ref_ty_generics {
            match self {
              #match_variants
            }
          }
        }
      });
    },

    Data::Union(_) => {
      return Err(Error::new(ast.span(), "Not for Union data type"))
    }
  }

  Ok(gen.into())
}


/// return (is-named, ref-fields token, unfolded-fields token)
pub fn get_ref_fields(fields: &Fields) -> (bool, TokenStream, TokenStream) {

  // ref fields
  let mut quote = TokenStream::new();
  // unfolded fields for matching
  let mut quote2 = TokenStream::new();

  match fields {
    Fields::Named(fields) => {
      let len = fields.named.len();
      for (i, field) in fields.named.iter().enumerate() {

        let vis = field.vis.to_token_stream();
        let name = field.ident.as_ref().unwrap();
        let ty = field.ty.to_token_stream();
        quote.extend( if i+1==len {
          quote! { #vis #name: &'a #ty }
        } else {
          quote! { #vis #name: &'a #ty, }
        });

        quote2.extend(if i+1==len { quote! { #name } } else { quote! { #name, } });
      }

      (true, quote! { {#quote} }, quote! { {#quote2} })
    },
    
    Fields::Unnamed(fields) => {
      let len = fields.unnamed.len();
      for (i, field) in fields.unnamed.iter().enumerate() {
        let vis = field.vis.to_token_stream();
        let ty = field.ty.to_token_stream();
        quote.extend( if i+1==len {
          quote! { #vis &'a #ty }
        } else {
          quote! { #vis &'a #ty, }
        });

        let name_ = format!("f{}", i);
        let name_ = Ident::new(&name_, fields.span());
        quote2.extend(if i+1==len { quote! { #name_ }} else { quote! { #name_, }})
      }

      
      (false, quote! { (#quote) }, quote! { (#quote2)})
    },

    Fields::Unit => {
      (false, quote, quote2)
    }
  }
}