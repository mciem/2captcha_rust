use std::ops::{Deref, Not};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, ConstParam,
    Field, FieldsNamed, GenericParam, Generics, LifetimeParam, PathArguments, Type, TypeParam,
};

pub struct ClassifiedFields<'a> {
    pub required: Vec<&'a Field>,
    pub optional: Vec<&'a Field>,
    pub phantoms: Vec<Field>,

    pub type_state_generics: Vec<GenericParam>,
}

impl<'a> ClassifiedFields<'a> {
    pub fn new(fields: &'a FieldsNamed, generics: &'a Generics) -> Self {
        use ConstParam as C;
        use GenericParam as G;
        use LifetimeParam as L;
        use TypeParam as T;

        let required = fields
            .named
            .iter()
            .filter(|&x| Self::is_optional_field(x).not())
            .collect::<Vec<_>>();

        let type_state_generics = ('T'..='Z')
            .chain('A'..'T')
            .skip(generics.type_params().count())
            .take(required.len())
            .map(|x| {
                G::Type(T {
                    ident: format_ident!("{x}"),
                    attrs: vec![],
                    bounds: Punctuated::default(),
                    default: None,
                    eq_token: None,
                    colon_token: None,
                })
            })
            .collect();

        Self {
            phantoms: generics
                .params
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    (
                        format_ident!("phantom_data_{i}"),
                        match x {
                            G::Lifetime(L { lifetime, .. }) => quote!(&#lifetime str),
                            G::Type(T { ident, .. }) => quote!(#ident),
                            G::Const(C { ident, .. }) => quote!([(), #ident]),
                        },
                    )
                })
                .map(|(i, x)| parse_quote!(#i: std::marker::PhantomData<#x>))
                .collect(),
            required,
            optional: fields
                .named
                .iter()
                .filter(|&x| Self::is_optional_field(x))
                .collect(),

            type_state_generics,
        }
    }

    pub fn self_optional_and_phantom(&self) -> TokenStream {
        let fields = self
            .phantoms
            .iter()
            .chain(self.optional.iter().map(Deref::deref))
            .map(|x| x.ident.as_ref().unwrap())
            .map(|x| quote! {#x: self.#x});

        quote! {
            #(#fields,)*
        }
    }

    pub fn build_fields(&self) -> TokenStream {
        let required = self
            .required
            .iter()
            .map(|x| x.ident.as_ref().unwrap())
            .map(|x| quote!(#x: self.#x.0));

        let optional = self
            .optional
            .iter()
            .map(|x| x.ident.as_ref().unwrap())
            .map(|x| quote!(#x: self.#x));

        quote!(
            #(#required,)*
            #(#optional,)*
        )
    }

    fn is_optional_field(field: &Field) -> bool {
        use AngleBracketedGenericArguments as A;
        use PathArguments::AngleBracketed as PA;

        match field.ty {
            Type::Path(ref ty) => {
                let Some(last_segment) = ty.path.segments.last() else {
                    return false;
                };

                last_segment.ident == "Option"
                    && matches!(
                        last_segment.arguments,
                        PA(A { ref args, .. }) if args.len() == 1
                    )
            }
            _ => false,
        }
    }
}

impl<'a> ToTokens for ClassifiedFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let comma = Comma {
            spans: [Span::call_site()],
        };

        for field in &self.phantoms {
            field.to_tokens(tokens);

            comma.to_tokens(tokens);
        }

        for (&field, ty) in self.required.iter().zip(&self.type_state_generics) {
            let mut field = field.clone();
            field.ty = parse_quote!(#ty);
            field.to_tokens(tokens);

            comma.to_tokens(tokens);
        }

        for field in &self.optional {
            field.to_tokens(tokens);

            comma.to_tokens(tokens);
        }
    }
}
