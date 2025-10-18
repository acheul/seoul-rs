use super::*;
use std::collections::HashSet;


pub(super) fn handle_fields(
    fields: &syn::Fields,
    //
    into_field: bool,
    intofrom_tuple: bool,
    //
    ast: &DeriveInput,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
    ref_impl_generics: &syn::ImplGenerics,
    gen: &mut TokenStream,
    lifetime: &syn::Lifetime,
) -> Result<()> {

    let type_name = &ast.ident;

    // (1) "into_field"
    let mut cache_types: HashSet<String> = HashSet::new();

    for (idx, field) in fields.iter().enumerate() {

        // * Check field's attribute and decide to skip or not
        let skip = {
            if let Some(attr) = get_attribute(&field.attrs, "into_field") {
                attr.parse_args::<syn::Path>().ok().map(|x| x.is_ident("skip")).unwrap_or(false)
            } else {
                !into_field
            }
        };
        if skip {
            continue
        }

        // Get `field_ty` and `field_name`.
        let field_ty = field.ty.to_token_stream();

        // * Check if the type already cached or not.
        if !cache_types.insert(field_ty.to_string()) {
            continue
        }
        let field_name = match field.ident.as_ref() {
            Some(name) => name.to_token_stream(),
            None => syn::Index::from(idx).to_token_stream()
        };

        gen.extend(quote! {
// Self Into Field
impl #impl_generics Into<#field_ty> for #type_name #ty_generics #where_clause {
    fn into(self) -> #field_ty {
        self.#field_name
    }
}
// &Self Into &Feild
impl #ref_impl_generics Into<&#lifetime #field_ty> for &#lifetime #type_name #ty_generics #where_clause {
    fn into(self) -> &#lifetime #field_ty {
        &self.#field_name
    }
}
// &mut Self Into &mut Field
impl #ref_impl_generics Into<&#lifetime mut #field_ty> for &#lifetime mut #type_name #ty_generics #where_clause {
    fn into(self) -> &#lifetime mut #field_ty {
        &mut self.#field_name
    }
}
        });
    }

    // (2) "intofrom_tuple"
    if intofrom_tuple {

        // Get helper tokens.
        let Some((
            tuple_args, build_token,
            (tuple_token, ref_tuple_token, ref_mut_tuple_token),
            (into_token, ref_into_token, ref_mut_into_token)
        )) = get_tokens_for_intofrom_tuple(&fields, lifetime)
        else {
            return Err(Error::new(ast.span(), "Isomorphism: Not for 0 fields data"));
        };

        gen.extend(quote! {
// Tuple Into Self
impl #impl_generics From<#tuple_token> for #type_name #ty_generics #where_clause {
    fn from(#tuple_args: #tuple_token) -> Self {
        Self #build_token
    }
}
// Self Into Tuple
impl #impl_generics Into<#tuple_token> for #type_name #ty_generics #where_clause {
    fn into(self) -> #tuple_token {
        #into_token
    }
}
// &Self Into &Tuple
impl #ref_impl_generics Into<#ref_tuple_token> for &#lifetime #type_name #ty_generics #where_clause {
    fn into(self) -> #ref_tuple_token {
        #ref_into_token
    }
}
// &mut Self Into &mut Tuple
impl #ref_impl_generics Into<#ref_mut_tuple_token> for &#lifetime mut #type_name #ty_generics #where_clause {
    fn into(self) -> #ref_mut_tuple_token {
        #ref_mut_into_token
    }
}
        });
    }

    Ok(())
}



/// Helper for `intofrom_tuple`
/// * return (
///     tuple-args,
///     build-token,
///     (tuple-token,
///     ref-tuple-token,
///     ref-mut-tuple-token),
///     (into-token,
///     ref-into-token,
///     ref-mut-into-token),
/// )
/// * return None if length==0
fn get_tokens_for_intofrom_tuple(
    fields: &Fields, lifetime: &syn::Lifetime
) -> Option<(
    TokenStream, TokenStream,
    (TokenStream, TokenStream, TokenStream),
    (TokenStream, TokenStream, TokenStream),
)> {
    let mut tuple_args = TokenStream::new();
    let mut build_token: TokenStream = TokenStream::new();

    let mut tuple_token: TokenStream = TokenStream::new();
    let mut ref_tuple_token: TokenStream = TokenStream::new();
    let mut ref_mut_tuple_token: TokenStream = TokenStream::new();
    
    let mut into_token: TokenStream = TokenStream::new();
    let mut ref_into_token: TokenStream = TokenStream::new();
    let mut ref_mut_into_token: TokenStream = TokenStream::new();
    

    let len = fields.len();
    if len == 0 {
        return None;
    }
    let is_named = if let Fields::Named(_) = fields {
        true
    } else {
        false
    };

    for (i, field) in fields.iter().enumerate() {

        let ty = field.ty.to_token_stream();
        tuple_token.extend(quote! { #ty, });
        ref_tuple_token.extend(quote! { &#lifetime #ty, });
        ref_mut_tuple_token.extend(quote! { &#lifetime mut #ty, });

        let index = syn::Index::from(i);
        let f = Ident::new(&format!("f{}", i), fields.span());
        tuple_args.extend(quote! { #f, });

        if let Some(name) = field.ident.as_ref() {
            build_token.extend(quote! { #name : #f, });
            into_token.extend(quote! { self . #name , });
            ref_into_token.extend(quote! { &self . #name , });
            ref_mut_into_token.extend(quote! { &mut self . #name , });
        } else {
            build_token.extend(quote! { #f , });
            into_token.extend(quote! { self . #index , });
            ref_into_token.extend(quote! { &self . #index , });
            ref_mut_into_token.extend(quote! { &mut self . #index , });
        }
    }

    tuple_args = quote! { (#tuple_args) };
    build_token = if is_named {
        quote! { { #build_token } }
    } else {
        quote! { (#build_token) }
    };
    tuple_token = quote! { (#tuple_token) };
    ref_tuple_token = quote! { (#ref_tuple_token) };
    ref_mut_tuple_token = quote! { (#ref_mut_tuple_token) };

    into_token = quote! { (#into_token) };
    ref_into_token = quote! { (#ref_into_token) };
    ref_mut_into_token = quote! { (#ref_mut_into_token) };

    Some((
        tuple_args, build_token,
        (tuple_token, ref_tuple_token, ref_mut_tuple_token),
        (into_token, ref_into_token, ref_mut_into_token),
    ))
}