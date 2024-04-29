use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse2, parse_quote, Error, Fields, FieldsNamed, ItemStruct, LitStr, MetaList, Path, Result,
};

use crate::macros::parsing::{
    captcha::{Captcha, ProxyTask},
    Attr, ContainerAttr,
};

use self::builder::Builder;

mod builder;

#[must_use]
pub fn captcha(args: TokenStream, item: &TokenStream) -> TokenStream {
    expand_captcha_attr(args, item).unwrap_or_else(|e| {
        let error = e.into_compile_error();
        quote! {
            #item
            #error
        }
    })
}

fn expand_captcha_attr(args: TokenStream, item: &TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<ItemStruct>(item.clone())?;

    let Fields::Named(ref mut fields) = input.fields else {
        return Err(Error::new(
            Span::call_site(),
            "Captcha can only be derived by structs with named fields",
        ));
    };

    let attr = Captcha::parse(args)?;

    if attr.timeout == 0 {
        return Err(Error::new(Span::call_site(), "Missing `timeout` attribute"));
    }

    if attr.solution.is_none() {
        return Err(Error::new(
            Span::call_site(),
            "Missing `solution` attribute",
        ));
    }

    let crate_rename = attr.crate_rename();
    let serde_rename = attr.serde_rename();

    let proxy = generate_proxy_mod(&crate_rename, &serde_rename, attr.proxy.as_ref());

    let serde_tag = input
        .attrs
        .iter()
        .filter(|x| x.path().is_ident("serde"))
        .filter_map(|x| x.meta.require_list().ok())
        .map(is_valid_serde_tag)
        .try_fold(false, |acc, cur| Ok::<_, Error>(acc || cur?))?;

    validate_task_type(proxy.as_ref(), serde_tag, fields)?;

    if proxy.is_some() {
        let Some(lifetime) = input.generics.lifetimes().next() else {
            return Err(Error::new(
                Span::call_site(),
                "The use of `#[captcha(proxy(...))]` requires a lifetime parameter",
            ));
        };

        fields.named.push(parse_quote!(
            #[serde(flatten)]
            proxy: Option<ProxyTask<#lifetime>>
        ));
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let timeout = attr.timeout;
    let solution = attr.solution();

    let Fields::Named(ref fields) = input.fields else {
        unreachable!("I had to do this because at this point I need the reference to be immutable")
    };

    let builder = Builder::new(&input, fields, &attr);
    let builder_ty = builder.assoc_type();

    Ok(quote! {
        #input

        #proxy

        #builder

        impl #impl_generics #crate_rename::captcha::Captcha for #ident #ty_generics #where_clause {
            type Solution = #solution;
            type Builder = #builder_ty;

            fn get_timeout(&self) -> std::time::Duration {
                std::time::Duration::from_secs(#timeout)
            }
        }
    })
}

fn validate_task_type(
    proxy: Option<&TokenStream>,
    serde_tag: bool,
    fields: &FieldsNamed,
) -> Result<()> {
    match (proxy, serde_tag) {
        (Some(_), true) => Err(Error::new(
            Span::call_site(),
            r#"`#[captcha(proxy(..))]` is not compatible with `#[serde(tag = "..")]`"#,
        )),
        (Some(_), false) => fields
            .named
            .iter()
            .find(|x| x.ident.as_ref().is_some_and(|ident| ident == "proxy"))
            .map_or(Ok(()), |x| {
                Err(Error::new_spanned(
                    x,
                    "The use of `#[captcha(proxy(...))]` does not allow a field called `proxy`, \
                    as it will be generated automatically",
                ))
            }),
        (None, true) => Ok(()),
        (None, false) => Err(Error::new(
            Span::call_site(),
            r#"Using either `#[captcha(proxy(with_proxy = "..", without_proxy = ".."))]` or `#[serde(tag = "type")]` is required"#,
        )),
    }
}

fn generate_proxy_mod(
    crate_rename: &Path,
    serde_rename: &Path,
    proxy: Option<&ProxyTask>,
) -> Option<TokenStream> {
    proxy.as_ref().map(|proxy| {
        let with_proxy = &*proxy.name_with_proxy;
        let without_proxy = &*proxy.name_without_proxy;
        quote!(
            use proxy::ProxyTask;

            #[doc(hidden)]
            mod proxy {
                #[derive(#serde_rename::Serialize)]
                #[serde(tag = "type")]
                #[doc(hidden)]
                pub enum ProxyTask<'a> {
                    #[serde(rename = #with_proxy)]
                    WithProxy(#crate_rename::proxy::Proxy<'a>),

                    #[serde(rename = #without_proxy)]
                    ProxyLess,
                }
            }
        )
    })
}

fn is_valid_serde_tag(list: &MetaList) -> Result<bool> {
    let mut has_tag = false;

    list.parse_nested_meta(|meta| {
        if !meta.path.is_ident("tag") {
            return Ok(());
        }

        has_tag = true;

        let litstr: LitStr = meta.value()?.parse()?;
        litstr.value().eq("type").then_some(()).ok_or_else(|| {
            Error::new(
                litstr.span(),
                r#"`#[serde(tag = "..")]` must have a value of "type""#,
            )
        })
    })?;

    Ok(has_tag)
}
