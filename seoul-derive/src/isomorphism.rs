use crate::*;

pub fn impl_isomorphism_macro(ast: &DeriveInput) -> Result<TokenStream> {

  let name = &ast.ident;

  // top level attrs
  let mut ty = None::<Ident>;
  let mut ty_list: Vec<Expr> = Vec::new();
  let mut list = None::<syn::ExprArray>;
  let mut has_default = false;

  if let Some(attr) = ast.attrs.iter().find(|x| x.path().is_ident("isomorphism")) {

    attr.parse_nested_meta(|meta| {

      if meta.path.is_ident("has_default") {
        has_default = true;

      } else if meta.path.is_ident("list") {
        let arg: syn::ExprArray = meta.value()?.parse()?;
        list.replace(arg);

      } else if meta.path.is_ident("into") {
        let args: syn::ExprArray = meta.value()?.parse()?;
        for arg in args.elems.into_iter() {
          ty_list.push(arg);
        }

      } else if let Some(arg) = meta.path.get_ident().map(|x| x.clone()) {
          ty.replace(arg);

      } else {
        return Err(Error::new(ast.span(), "Top level 'isomorphism' attribute has argments of list or Into/From type."));
      }
      Ok(())
    })?;
  }

  // either or neither of ty or ty_list
  if ty.is_some() && !ty_list.is_empty() {
    return Err(Error::new(ast.span(), "To pass Into/From type, should either use format of simple ident or array."));
  }

  // get enum data
  let data = match &ast.data {
    Data::Enum(data) => {
      data
    },
    _ => return Err(Error::new(ast.span(), "Only for Enum data type.")),
  };

  // fallback list
  let mut fallback_list: Option<TokenStream> = if list.is_none() { Some(TokenStream::new()) } else { None };

  // build TokenStream touring each variants

  let len = if ty.is_some() {1} else {ty_list.len()};
  let mut quoted_into_list: Vec<TokenStream> = (0..len).map(|_| TokenStream::new()).collect();
  let mut quoted_from_list: Vec<TokenStream> = (0..len).map(|_| TokenStream::new()).collect();
  let mut quoted_title  = TokenStream::new();


  for variant in data.variants.iter() {

    let matching_format = variant_matching_format(name, variant)?;
    let default_format = variant_default_format(name, variant)?;

    fallback_list.as_mut().map(|x| x.extend(quote! { #default_format, }));

    let mut values: Vec<Expr> = Vec::new();
    let mut title = None::<Expr>;

    for attr in variant.attrs.iter() {

      if attr.path().is_ident("into") {
        if ty.is_some() {
          let arg: Expr = attr.parse_args()?;
          values.push(arg);
        
        } else if !ty_list.is_empty() {
          let args: syn::ExprArray = attr.parse_args()?;
          for arg in args.elems.into_iter() {
            values.push(arg);
          }
        }

      } else if attr.path().is_ident("title") {
        let arg: Expr = attr.parse_args()?;
        title.replace(arg);
      }
    };

    // Into, From
    if !ty_list.is_empty() {
      if ty_list.len() != values.len() {
        return Err(Error::new(ast.span(), "Using Into/From type arrays, should not abbreviate values."));
      }
    }

    // Ident ty
    if let Some(ty) = ty.as_ref() {
      // Into
      quoted_into_list.get_mut(0).map(|x| x.extend(
        if let Some(value) = values.first() {
          quote! { #matching_format => #value, }
        } else {
          quote! { #matching_format => #ty::default(), }
        }
      ));

      // From
      if has_default {
        if let Some(value) = values.first() {
          quoted_from_list.get_mut(0).map(|x| x.extend(
            quote! { #value => #default_format, }
          ));
        };
      }

    // List ty
    } else if !ty_list.is_empty() {
      for (quoted_into, (quoted_from, value)) in quoted_into_list.iter_mut().zip(quoted_from_list.iter_mut().zip(values.iter())) {
        // Into
        quoted_into.extend(
          quote! { #matching_format => #value, }
        );
        if has_default {
          quoted_from.extend(
            quote! { #value => #default_format, }
          );
        }
      }
    }

    // title
    quoted_title.extend(
      if let Some(title) = title {
        quote! { #matching_format =>  #title, }
      } else {
        let variant_name = &variant.ident.to_string();
        quote! {#matching_format => #variant_name, }
      }
    );
  };


  // finialize traits
  let mut quoted = TokenStream::new();

  // Into & From
  let impl_into_from = move |quoted: &mut TokenStream, quoted_into: TokenStream, quoted_from: TokenStream, ty: TokenStream| {
    // Into
    quoted.extend(quote! {

      impl<'a> Into<#ty> for &'a #name {
        fn into(self) -> #ty {
          match self {
            #quoted_into
          }
        }
      }

      impl Into<#ty> for #name {
        fn into(self) -> #ty {
          match self {
            #quoted_into
          }
        }
      }
    });
    // From
    if has_default {
      quoted.extend(quote! {

        impl From<#ty> for #name {
          fn from(value: #ty) -> Self {
            #[allow(unreachable_patterns)]
            match value {
              #quoted_from
              _ => #name::default()
            }
          }
        }
  
        impl<'a> From<&'a #ty> for #name {
          fn from(value: &'a #ty) -> Self {
            #[allow(unreachable_patterns)]
            match value {
              #quoted_from
              _ => #name::default()
            }
          }
        }
      });
    }
  };

  if let Some(ty) = ty {
    let quoted_into = quoted_into_list.remove(0);
    let quoted_from = quoted_from_list.remove(0);
    impl_into_from(&mut quoted, quoted_into, quoted_from, quote! { #ty });
  } else if !ty_list.is_empty() {
    for (quoted_into, (quoted_from, ty)) in quoted_into_list.into_iter().zip(quoted_from_list.into_iter().zip(ty_list.into_iter())) {
      impl_into_from(&mut quoted, quoted_into, quoted_from, quote! { #ty });
    }
  }

  // Isomorphism trait

  // list
  let mut quoted_list = TokenStream::new();

  if let Some(list) = list {
    for expr in list.elems.iter() {
      quoted_list.extend(
        quote! { Self::#expr, }
      );
    }
  } else {
    quoted_list = fallback_list.unwrap();
  }

  // list, title
  quoted.extend(quote! {
    impl Isomorphism for #name {
      fn title(&self) -> &str {
        match self {
          #quoted_title
        }
      }
      fn list() -> Vec<Self> {
        vec![#quoted_list]
      }
    }
  });

  Ok(quoted.into())
}