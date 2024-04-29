use syn::{parse_quote, Error, Path, Result, Type};

use crate::{
    macros::parsing::{
        assignment::{assign, assign_int, assign_str},
        Attr, ContainerAttr,
    },
    parse_attrs,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ProxyTask {
    pub name_with_proxy: String,
    pub name_without_proxy: String,
}

#[derive(Debug, Default)]
pub struct Captcha {
    pub crate_rename: Option<Path>,
    pub serde_rename: Option<Path>,
    pub proxy: Option<ProxyTask>,
    pub timeout: u64,
    pub solution: Option<Type>,
}

impl Captcha {
    pub fn solution(&self) -> Type {
        self.solution.clone().unwrap()
    }
}

impl<'a> ContainerAttr<'a> for Captcha {
    fn crate_rename(&self) -> Path {
        self.crate_rename
            .clone()
            .unwrap_or_else(|| parse_quote!(::captcha_oxide))
    }

    fn serde_rename(&self) -> Path {
        self.serde_rename
            .clone()
            .unwrap_or_else(|| parse_quote!(::serde))
    }
}

parse_attrs! {
    Captcha(input, output) {
        "crate": output.crate_rename.is_some() => {
            output.crate_rename = Some(assign(&input)?);
        },
        "serde": output.serde_rename.is_some() => {
            output.serde_rename = Some(assign(&input)?);
        },
        "proxy": output.proxy.is_some() => {
            let proxy = ProxyTask::parse(&input)?;

            if proxy.name_with_proxy.is_empty() {
                return Err(input.error("`with_proxy` is required"))
            }

            if proxy.name_without_proxy.is_empty() {
                return Err(input.error("`without_proxy` is required"))
            }

            output.proxy = Some(proxy);
        },
        "timeout": output.timeout != 0 => {
            let timeout = assign_int(&input)?.base10_parse()?;

            if timeout == 0 {
                return Err(input.error("`timeout` must not be 0"))
            }

            output.timeout = timeout;
        },
        "solution": output.solution.is_some() => {
            output.solution = Some(assign(&input)?);
        },
    }
}

parse_attrs! {
    nested ProxyTask(input, output) {
        "with_proxy": !output.name_with_proxy.is_empty() => {
            let string = assign_str(&input)?;
            let value = string.value();

            if value.is_empty() {
                return Err(Error::new(string.span(), "`with_proxy` must not be empty"));
            }

            output.name_with_proxy = value;
        },
        "without_proxy": !output.name_without_proxy.is_empty() => {
            let string = assign_str(&input)?;
            let value = string.value();

            if value.is_empty() {
                return Err(Error::new(string.span(), "`without_proxy` must not be empty"));
            }

            output.name_without_proxy = value;
        }
    }
}
