use std::ops::Not;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_quote, FieldsNamed, GenericArgument, GenericParam, Generics, Ident, ItemStruct, Type,
};

use crate::{ext::generic_param::GenericParamExt, macros::parsing::captcha::Captcha};

use self::{classified_fields::ClassifiedFields, type_state::TypeState};

mod classified_fields;
mod type_state;

pub struct Builder<'a> {
    item: &'a ItemStruct,
    ident: Ident,
    classified_fields: ClassifiedFields<'a>,
    type_state: TypeState,
}

impl<'a> Builder<'a> {
    pub fn new(item: &'a ItemStruct, fields: &'a FieldsNamed, _a: &Captcha) -> Self {
        let ident = format_ident!("{}Builder", item.ident);

        let classified_fields = ClassifiedFields::new(fields, &item.generics);
        let type_state = TypeState::new(
            &classified_fields.required,
            &item.generics,
            classified_fields.type_state_generics.clone(),
        );

        Self {
            item,
            ident,
            classified_fields,
            type_state,
        }
    }

    pub fn assoc_type(&self) -> Type {
        let ident = &self.ident;
        let ty_generics = self
            .item
            .generics
            .params
            .iter()
            .map(GenericParamExt::as_generic_arg)
            .chain(self.type_state.missing_structs.iter().map(|x| {
                let ident = &x.ident;
                GenericArgument::Type(parse_quote!(builder::#ident))
            }))
            .collect::<Vec<_>>();

        let ty_generics = ty_generics
            .is_empty()
            .not()
            .then_some(quote!(<#(#ty_generics),*>));

        parse_quote!(builder::#ident #ty_generics)
    }

    fn all_generics(&self) -> Generics {
        let mut generics = self.item.generics.clone();
        for generic in &self.classified_fields.type_state_generics {
            generics.params.push(generic.clone());
        }

        generics
    }

    fn fields_initial_value(&self) -> TokenStream {
        let mut tokens = quote! {};

        for field in &self.classified_fields.phantoms {
            let ident = field.ident.as_ref().unwrap();
            quote! { #ident: std::marker::PhantomData, }.to_tokens(&mut tokens);
        }

        let required_fields = self.classified_fields.required.iter();
        let missing_structs = self.type_state.missing_structs.iter();
        for (field, missing_struct) in required_fields.zip(missing_structs) {
            let ident = field.ident.as_ref().unwrap();
            let ty = &missing_struct.ident;
            quote! { #ident: #ty, }.to_tokens(&mut tokens);
        }

        for field in &self.classified_fields.optional {
            let ident = field.ident.as_ref().unwrap();
            quote! { #ident: None, }.to_tokens(&mut tokens);
        }

        tokens
    }

    fn default_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let (impl_generics, _, where_clause) = self.item.generics.split_for_impl();

        let ty_generics = self
            .item
            .generics
            .params
            .iter()
            .map(GenericParamExt::as_generic_arg)
            .chain(self.type_state.missing_structs.iter().map(|x| {
                let ident = &x.ident;
                GenericArgument::Type(parse_quote!(#ident))
            }))
            .collect::<Vec<_>>();

        let ty_generics = ty_generics
            .is_empty()
            .not()
            .then_some(quote!(<#(#ty_generics),*>));

        let fields = self.fields_initial_value();

        quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                pub const fn new() -> Self {
                    Self {
                        #fields
                    }
                }
            }

            impl #impl_generics Default for #ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    }

    fn required_impl(&self) -> TokenStream {
        let Self {
            ref ident,
            ref classified_fields,
            ref type_state,
            ..
        } = self;

        let generics = self.all_generics();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let required_methods = classified_fields
            .required
            .iter()
            .zip(type_state.generated_generics.iter())
            .zip(type_state.provided_structs.iter())
            .map(|((a, b), c)| (a, b, c))
            .map(|(field, generic, provided_struct)| {
                let field_ident = field.ident.as_ref().unwrap();
                let required_fields = classified_fields
                    .required
                    .iter()
                    .map(|x| x.ident.as_ref().unwrap())
                    .filter(|&x| x != field_ident)
                    .map(|x| quote!(#x: self.#x));

                let docs = field.attrs.iter()
                    .filter(|attr| attr.path().is_ident("doc"));

                let struct_ident = &provided_struct.ident;
                let (_, struct_generics, _) = provided_struct.generics.split_for_impl();
                let generic_args = generics
                    .params
                    .clone()
                    .iter_mut()
                    .map(|x| match x {
                        GenericParam::Type(_) if x == generic => {
                            GenericArgument::Type(parse_quote!(#struct_ident #struct_generics))
                        }
                        _ => x.as_generic_arg(),
                    })
                    .collect::<Vec<_>>();

                let generic_args = generic_args
                    .is_empty()
                    .not()
                    .then_some(quote!(<#(#generic_args),*>));

                let self_fields = classified_fields.self_optional_and_phantom();

                let ty = &field.ty;

                quote! {
                    #[must_use]
                    #(#docs)*
                    pub fn #field_ident(self, #field_ident: impl Into<#ty>) -> #ident #generic_args {
                        #ident {
                            #field_ident: #struct_ident(#field_ident.into()),
                            #self_fields
                            #(#required_fields,)*
                        }
                    }
                }
            });

        quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #(#required_methods)*
            }
        }
    }

    fn optional_impl(&self) -> TokenStream {
        use syn::AngleBracketedGenericArguments as A;
        use syn::PathArguments::AngleBracketed as PA;

        let ident = &self.ident;
        let generics = self.all_generics();
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let methods = self.classified_fields.optional.iter().filter_map(|field| {
            let ident = field.ident.as_ref()?;
            let remove_ident = format_ident!("remove_{ident}");

            let docs = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("doc"));

            let Type::Path(ref ty_path) = field.ty else {
                return None;
            };

            let PA(A { ref args, .. }) = ty_path.path.segments.last()?.arguments else {
                return None;
            };

            let ty = args.first()?;

            Some(quote! {
                #[must_use]
                #(#docs)*
                pub fn #ident(mut self, #ident: impl Into<#ty>) -> Self {
                    self.#ident = Some(#ident.into());
                    self
                }

                #[must_use]
                pub fn #remove_ident(mut self) -> Self {
                    self.#ident = None;
                    self
                }
            })
        });

        quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #(#methods)*
            }
        }
    }

    fn build_impl(&self) -> TokenStream {
        let ident = &self.ident;
        let task_ident = &self.item.ident;
        let (impl_generics, task_ty_generics, where_clause) = self.item.generics.split_for_impl();

        let ty_generics = self
            .item
            .generics
            .params
            .iter()
            .map(GenericParamExt::as_generic_arg)
            .chain(self.type_state.provided_structs.iter().map(|x| {
                let ident = &x.ident;
                let (_, ty, _) = x.generics.split_for_impl();
                GenericArgument::Type(parse_quote!(#ident #ty))
            }))
            .collect::<Vec<_>>();

        let ty_generics = ty_generics
            .is_empty()
            .not()
            .then_some(quote!(<#(#ty_generics),*>));

        let fields = self.classified_fields.build_fields();

        quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn build(self) -> #task_ident #task_ty_generics {
                    #task_ident {
                        #fields
                    }
                }
            }
        }
    }
}

impl<'a> ToTokens for Builder<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ref ident,
            ref classified_fields,
            ref type_state,
            ..
        } = self;

        let all_generics = self.all_generics();

        let required_impl = self.required_impl();
        let optional_methods = self.optional_impl();

        let (_, ty_generics, where_clause) = all_generics.split_for_impl();

        let default_impl = self.default_impl();

        let build_impl = self.build_impl();

        quote! {
            mod builder {
                use super::*;

                mod type_state {
                    use super::*;
                    #type_state
                }

                pub use type_state::*;
                pub struct #ident #ty_generics #where_clause {
                    #classified_fields
                }

                #default_impl

                #required_impl

                #optional_methods

                #build_impl
            }
        }
        .to_tokens(tokens);
    }
}
