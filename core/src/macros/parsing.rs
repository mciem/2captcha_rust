use syn::{Path, Result};

pub mod assignment;

pub mod captcha;

pub trait Attr<'a>: Default + Sized {
    type From;
    fn parse(attrs: Self::From) -> Result<Self>;
}

pub trait ContainerAttr<'a>: Attr<'a> {
    fn crate_rename(&self) -> Path;
    fn serde_rename(&self) -> Path;
}

#[macro_export]
macro_rules! parse_attrs {
    ($i: ident ($input: ident, $output: ident) { $($k: literal $(: $duplicate: expr)? => $e:expr),* $(,)?  }) => {
        impl<'a> Attr<'a> for $i {
            type From = proc_macro2::TokenStream;
            fn parse(attrs: Self::From) -> Result<Self> {
                let attr: syn::Attribute = syn::parse_quote!(#[attr(#attrs)]);

                let mut $output = Self::default();

                attr.parse_nested_meta(|$input| {
                    $(
                        if $input.path.is_ident($k) {
                            $(
                                if $duplicate {
                                    return Err($input.error(
                                        format!(r#"Duplicate attribute `{}`"#, $k)
                                    ));
                                }
                            )?
                            $e
                        }
                    ) else *
                    else {
                        let ident = &$input.path.segments.last().unwrap().ident;
                        return Err($input.error(
                            format!(
                                "Unknown attribute \"{ident}\". Allowed attributes are: {}",
                                [$(stringify!($k),)*].join(", ")
                            )
                        ))
                    }

                    Ok(())
                })?;

                Ok($output)
            }
        }
    };
    (nested $i: ident ($input: ident, $output: ident) { $($k: literal $(: $duplicate: expr)? => $e:expr),* $(,)?  }) => {
        impl<'a> Attr<'a> for $i {
            type From = &'a syn::meta::ParseNestedMeta<'a>;
            fn parse($input: Self::From) -> Result<Self> {
                let mut $output = Self::default();

                $input.parse_nested_meta(|$input| {
                    $(
                        if $input.path.is_ident($k) {
                            $(
                                if $duplicate {
                                    return Err($input.error(
                                        format!(r#"Duplicate attribute `{}`"#, $k)
                                    ));
                                }
                            )?
                            $e
                        }
                    ) else *
                    else {
                        let ident = &$input.path.segments.last().unwrap().ident;
                        return Err($input.error(
                            format!(
                                "Unknown attribute \"{ident}\". Allowed attributes are: {}",
                                [$(stringify!($k),)*].join(", ")
                            )
                        ))
                    }

                    Ok(())
                })?;

                Ok($output)
            }
        }
    };
}
