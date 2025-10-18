use crate::*;
mod fields;
mod variants;


pub(super) fn impl_isomorphism_macro(ast: &DeriveInput) -> Result<TokenStream> {

    let mut gen = TokenStream::new();

    // parsing generics
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    
    // Get lifetime added impl_generics
    let lifetime: syn::Lifetime = syn::Lifetime::new("'iso", Span::call_site());

    let mut _generics = ast.generics.clone();
    let (ref_impl_generics, ..) = {
        let ltp = syn::LifetimeParam::new(lifetime.clone());
        _generics.params.push(syn::GenericParam::from(ltp));

        _generics.split_for_impl()
    };


    // 1. Container attributes

    let mut into_field = false;
    let mut intofrom_tuple = false;

    let mut from_variant = false;
    let mut name_variant_method = None::<String>;
    
    let mut into_type = None::<Expr>;
    let mut default_into_value = None::<Expr>;
    let mut skip_restore = false;
    let mut skip_ref_restore = false;
    let mut restore_panic = None::<String>;
    let mut default_restore_value = None::<Expr>;

    
    if let Some(attr) = ast.attrs.iter().find(|attr| attr.path().is_ident("isomorphism"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("into_field") {
                into_field = true;
            } else if meta.path.is_ident("intofrom_tuple") {
                intofrom_tuple = true;

            } else if meta.path.is_ident("from_variant") {
                from_variant = true;
            } else if meta.path.is_ident("name_variant") {
                let method_name = if let Ok(x) = meta.value() {
                    x.parse::<syn::LitStr>()?.value()
                } else {
                    "name".to_string()
                };
                name_variant_method.replace(method_name);

            } else if meta.path.is_ident("into") {
                let expr = meta.value()?.parse::<Expr>()?;
                into_type.replace(expr);
            } else if meta.path.is_ident("default_into") {
                let expr = meta.value()?.parse::<Expr>()?;
                default_into_value.replace(expr);
            } else if meta.path.is_ident("skip_restore") {
                skip_restore = true;
            } else if meta.path.is_ident("skip_ref_restore") {
                skip_ref_restore = true;
            } else if meta.path.is_ident("restore_panic") {
                let lit = meta.value()?.parse::<syn::LitStr>()?;
                restore_panic.replace(lit.value());

            } else if meta.path.is_ident("default_restore") {
                let expr = meta.value()?.parse::<Expr>()?;
                default_restore_value.replace(expr);
            
            } else {
                let x = meta.path.get_ident().map(|ident| ident.to_string());
                return Err(Error::new(ast.span(), &format!("Isomorphism: Path {:?} is not allowed for attribute Isomorphism.", x)));
            }
            Ok(())
        })?;
    }
    

    // 2. On each field/variant.
    match &ast.data {
        // 1) On fields
        Data::Struct(data) => {
            let _ = fields::handle_fields(
                &data.fields, into_field, intofrom_tuple,
                ast, &impl_generics, &ty_generics, &where_clause,
                &ref_impl_generics,
                &mut gen, &lifetime
            )?;
        },

        // 2) On Enum
        Data::Enum(data) => {
            
            if into_field {
                return Err(Error::new(ast.span(), "Isomorphism: `into_field` attribute is not for Enum data type.")); 
            } else if intofrom_tuple {
                return Err(Error::new(ast.span(), "Isomorphism: `intofrom_tuple` attribute is not for Enum data type.")); 
            }

            let _ = variants::handle_variants(
                &data.variants,
                from_variant, name_variant_method, 
                into_type, default_into_value, skip_restore, skip_ref_restore, restore_panic, default_restore_value,                
                ast, &impl_generics, &ty_generics, &where_clause, &ref_impl_generics, 
                &mut gen, &lifetime
            )?;
        },

        // 3) On Union:
        Data::Union(_) => {
            return Err(Error::new(ast.span(), "Cannot implement for Union data type."));
        },
    }

    Ok(gen.into())
}


// Common helpers
fn fields_default_format(fields: &syn::Fields) -> TokenStream {
    let quoted = match fields {
        Fields::Named(fields) => {
            let mut quoted = TokenStream::new();
            let len = fields.named.len();

            for (i, field) in fields.named.iter().enumerate() {
                let name = field.ident.as_ref().unwrap();
                let x = if i + 1 == len {
                    quote! { #name: Default::default() }
                } else {
                    quote! { #name: Default::default(), }
                };
                quoted.extend(x);
            }

            quote! {
              {#quoted}
            }
        }
        Fields::Unnamed(fields) => {
            let mut quoted = TokenStream::new();
            let len = fields.unnamed.len();

            for (i, _field) in fields.unnamed.iter().enumerate() {
                let x = if i + 1 == len {
                    quote! { Default::default() }
                } else {
                    quote! { Default::default(), }
                };
                quoted.extend(x);
            }

            quote! {
              (#quoted)
            }
        }
        Fields::Unit => TokenStream::new(),
    };

    quoted
}

fn variant_matching_format(type_name: &Ident, variant: &syn::Variant) -> TokenStream {

    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Named(_) => quote! { #type_name :: #variant_name {..} },
        Fields::Unnamed(_) => quote! { #type_name :: #variant_name(..) },
        Fields::Unit => quote! { #type_name :: #variant_name },
    }
}

/// Parse attribute of a field or a variant.
fn get_attribute<'a>(attrs: &'a [syn::Attribute], path: &str) -> Option<&'a syn::Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident(path))
}