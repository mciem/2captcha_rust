use quote::{format_ident, ToTokens};
use syn::{
    parse_quote, AngleBracketedGenericArguments, Field, GenericArgument, GenericParam, Generics,
    PathArguments, PredicateLifetime, PredicateType, Type, TypeArray, TypeParen, TypePtr,
    TypeReference, TypeSlice, TypeTuple,
};

use crate::ext::ident::IdentExt;

pub struct TypeState {
    pub provided_structs: Vec<syn::ItemStruct>,
    pub missing_structs: Vec<syn::ItemStruct>,

    pub generated_generics: Vec<GenericParam>,
}

impl TypeState {
    pub fn new(
        fields: &[&Field],
        original_generics: &Generics,
        type_state_generics: Vec<GenericParam>,
    ) -> Self {
        use syn::WherePredicate::{Lifetime as WL, Type as WT};

        Self {
            missing_structs: fields
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().unwrap().to_pascal_case();

                    let missing_ident = format_ident!("Missing{ident}");

                    parse_quote!(
                        #[derive(Default)]
                        pub struct #missing_ident;
                    )
                })
                .collect(),
            provided_structs: fields
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().unwrap().to_pascal_case();

                    let provided_ident = format_ident!("{ident}Provided");

                    let generics = extract_generics(&field.ty, original_generics);
                    let where_clause = original_generics.where_clause.clone().as_mut().map(|x| {
                        x.predicates = x
                            .predicates
                            .iter()
                            .filter(|p| match p {
                                WL(PredicateLifetime { ref lifetime, .. }) => {
                                    generics.iter().any(|g| match g {
                                        GenericArgument::Lifetime(ref l) => {
                                            l.ident == lifetime.ident
                                        }
                                        _ => false,
                                    })
                                }
                                WT(PredicateType { ref bounded_ty, .. }) => {
                                    generics.iter().any(|g| match g {
                                        GenericArgument::Type(ref t) => t == bounded_ty,
                                        _ => false,
                                    })
                                }
                                _ => false,
                            })
                            .cloned()
                            .collect();

                        x.clone()
                    });

                    let ty = &field.ty;
                    let provided = parse_quote!(
                        pub struct #provided_ident<#(#generics),*>(pub #ty) #where_clause;
                    );

                    provided
                })
                .collect(),

            generated_generics: type_state_generics,
        }
    }
}

impl ToTokens for TypeState {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for struct_def in &self.missing_structs {
            struct_def.to_tokens(tokens);
        }
        for struct_def in &self.provided_structs {
            struct_def.to_tokens(tokens);
        }
    }
}

fn extract_generics(ty: &Type, generics: &Generics) -> Vec<GenericArgument> {
    use AngleBracketedGenericArguments as A;
    use GenericArgument::Type as GAT;
    use PathArguments::AngleBracketed as PA;
    use PathArguments::None as PN;

    match ty {
        Type::Path(ref ty_path) => ty_path
            .path
            .segments
            .iter()
            .filter_map(|segment| match segment.arguments {
                PA(A { ref args, .. }) => Some(args),
                _ => None,
            })
            .flat_map(|args| {
                args.iter().filter(|x| match x {
                    GAT(Type::Path(ty_path)) if ty_path.path.segments.len() == 1 => {
                        let segment = ty_path.path.segments.first().unwrap();

                        match segment.arguments {
                            PN => generics.type_params().any(|x| x.ident == segment.ident),
                            _ => false,
                        }
                    }
                    GAT(_) => false,
                    _ => true,
                })
            })
            .cloned()
            .collect(),
        Type::Paren(TypeParen { elem, .. })
        | Type::Ptr(TypePtr { elem, .. })
        | Type::Slice(TypeSlice { elem, .. })
        | Type::Array(TypeArray { elem, .. }) => extract_generics(elem, generics),
        Type::Tuple(TypeTuple { elems, .. }) => elems
            .iter()
            .flat_map(|x| extract_generics(x, generics))
            .collect(),
        Type::Reference(TypeReference { elem, lifetime, .. }) => {
            let mut generics = extract_generics(elem, generics);

            if let Some(lifetime) = lifetime.as_ref() {
                let needs_lifetime = generics.iter().all(|x| match x {
                    GenericArgument::Lifetime(l) => l.ident != lifetime.ident,
                    _ => false,
                });

                if needs_lifetime {
                    let mut g = Vec::with_capacity(generics.len() + 1);
                    g.push(GenericArgument::Lifetime(lifetime.clone()));
                    g.extend(generics);
                    generics = g;
                }
            }

            generics
        }
        _ => vec![],
    }
}
