use syn::{meta::ParseNestedMeta, LitStr, Result};

pub fn assign_str(input: &ParseNestedMeta) -> Result<LitStr> {
    let value = input.value()?;
    value.parse()
}
