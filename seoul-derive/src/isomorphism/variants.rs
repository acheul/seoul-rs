use super::*;
use std::collections::HashSet;


pub(super) fn handle_variants(
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    //
    from_variant: bool,
    name_variant_method: Option<String>,
    into_type: Option<Expr>,
    default_into_value: Option<Expr>,
    skip_restore: bool,
    skip_ref_restore: bool,
    restore_panic: Option<String>,
    default_restore_value: Option<Expr>,
    //
    ast: &DeriveInput,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
    ref_impl_generics: &syn::ImplGenerics,
    gen: &mut TokenStream, 
    lifetime: &syn::Lifetime
) -> Result<()>
{
    let type_name = &ast.ident;

    // (1) "from_variant"
    let mut cache_types: HashSet<String> = HashSet::new();

    for variant in variants.iter() {

        // * Check field's attribute and decide to skip or not
        let skip = {
            if let Some(attr) = get_attribute(&variant.attrs, "from_variant") {
                attr.parse_args::<syn::Path>().ok().map(|x| x.is_ident("skip")).unwrap_or(false)
            } else {
                !from_variant
            }
        };
        if skip {
            continue
        }

        // Get tokens.
        let var_name = &variant.ident;
        let (from_type, from_arg, build_token) = get_tokens_for_from_variant(&variant.fields);

        // * Check if the type already cached or not.
        if !cache_types.insert(from_type.to_string()) {
            continue
        }
        
        gen.extend(quote! {

impl #impl_generics From<#from_type> for #type_name #ty_generics #where_clause {
    fn from(#from_arg: #from_type) -> Self {
        Self :: #var_name #build_token
    }
}
        });
    }

    // (2) "name_variant_method"
    if let Some(method_name) = name_variant_method {
        let method_name = Ident::new(&method_name, Span::call_site());

        let mut match_names = TokenStream::new();

        for variant in variants.iter() {
        
            // Get matching token
            let matching = variant_matching_format(type_name, variant);

            match get_attribute(&variant.attrs, "name") {
                Some(attr) => {
                    let name = attr.parse_args::<Expr>()?;
                    match_names.extend(quote! { #matching => #name, });
                },
                None => {
                    let name = &variant.ident.to_string();
                    match_names.extend(quote! { #matching => #name, });
                }
            };
        }

        gen.extend(quote! {

impl #impl_generics #type_name #ty_generics #where_clause {
    pub fn #method_name(&self) -> &str {
        match self {
            #match_names
        }
    }
}
        });
    }

    // (3) "convert"
    if let Some(into_type) = into_type {

        let mut match_into = TokenStream::new();
        let mut match_from = TokenStream::new();

        for variant in variants.iter() {

            let variant_name = &variant.ident;

            // 1) Extend "Into":
            // matching token
            let matching = variant_matching_format(type_name, variant);

            // convert-into-value
            let into_value = match get_attribute(&variant.attrs, "into") {
                Some(attr) => Some(attr.parse_args::<Expr>()?),
                None => default_into_value.clone()
            };

            match into_value.as_ref() {
                Some(expr) => {
                    match_into.extend(quote! { #matching => #expr, });
                },
                None => {
                    match_into.extend(quote! { #matching => #into_type::default(), });
                },
            }

            // 2) Extend "From"(restore):
            if let Some(into_value) = into_value {
                match get_attribute(&variant.attrs, "restore") {
                    Some(attr) => {
                        let expr = attr.parse_args::<Expr>()?;
                        match_from.extend(quote! { #into_value => #expr, });
                    },
                    None => {
                        let fields_format = fields_default_format(&variant.fields);
                        match_from.extend(quote! { #into_value => #type_name :: #variant_name #fields_format, });
                    },
                };
            }
        }

        // 3) Fallback of "From"(restore)
        match restore_panic {
            Some(expr) => {
                match_from.extend(quote! { _ => panic!(#expr) });
            },
            None => {
                match default_restore_value {
                    Some(expr) => {
                        match_from.extend(quote! { _ => #expr });
                    },
                    None => {
                        match_from.extend(quote! { _ => #type_name::default() });
                    },
                }
            }
        }

        // Extend to gen
            gen.extend(quote! {
// Self => ConvertType
impl #impl_generics Into<#into_type> for #type_name #ty_generics #where_clause {
    fn into(self) -> #into_type {
        match self {
            #match_into
        }
    }
}
// &Self => ConvertType
impl #ref_impl_generics Into<#into_type> for &#lifetime #type_name #ty_generics #where_clause {
    fn into(self) -> #into_type {
        match self {
            #match_into
        }
    }
}
        });

        // Extend convert-from when `skip_restore` is false.
        if !skip_restore {
            gen.extend(quote! {
// ConvertType => Self
impl #impl_generics From<#into_type> for #type_name #ty_generics #where_clause {
    fn from(value: #into_type) -> Self {
        #[allow(unreachable_patterns)]
        match value {
            #match_from
        }
    }
}
            });
        }

        // Extend ref convert-from when `skip_restore` and `skip_ref_restore` is false.
        if !skip_restore && !skip_ref_restore {
            gen.extend(quote! {
// &ConvertType => Self
impl #ref_impl_generics From<&#lifetime #into_type> for #type_name #ty_generics #where_clause {
    fn from(value: &#lifetime #into_type) -> Self {
        #[allow(unreachable_patterns)]
        match value {
            #match_from
        }
    }
}
            });
        }
    }

    Ok(())
}


/// (from-type, from-arg, build-token)
fn get_tokens_for_from_variant(fields: &Fields) -> (TokenStream, TokenStream, TokenStream)
{
    let mut from_type = TokenStream::new();
    let mut from_arg = TokenStream::new();
    let mut build_token = TokenStream::new();

    if fields.is_empty() {
        from_type = quote! { () };
        from_arg = quote! { _ };
    
    } else if fields.len() == 1 {
        let field = fields.iter().next().unwrap();
        let field_ty = field.ty.to_token_stream();
            
        from_type = quote! { #field_ty };
        from_arg = quote! { value };
        build_token = if let Some(name) = field.ident.as_ref() {
            quote! { #name: value }
        } else {
            quote! { value }
        };
    
    } else {

        for (i, field) in fields.iter().enumerate() {

            let ty = field.ty.to_token_stream();
            from_type.extend(quote! { #ty, });

            let f = Ident::new(&format!("f{}", i), fields.span());
            from_arg.extend(quote! { #f, });

            if let Some(name) = field.ident.as_ref() {
                build_token.extend(quote! { #name: #f, });
            } else {
                build_token.extend(quote! { #f, });
            }
        }

        from_type = quote! { (#from_type) };
        from_arg = quote! { (#from_arg) };
    }


    build_token = match fields {
        Fields::Named(_) => quote! { { #build_token } },
        Fields::Unnamed(_) => quote! { ( #build_token ) },
        Fields::Unit => build_token,
    };
    
    (from_type, from_arg, build_token)
}