use syn::{meta::ParseNestedMeta, parse::Parse, LitStr, Result};

pub fn assign<T>(input: &ParseNestedMeta) -> Result<T>
where
    T: Parse,
{
    let value = input.value()?;
    let string: LitStr = value.parse()?;

    string.parse()
}
