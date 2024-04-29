use syn::{meta::ParseNestedMeta, LitInt, Result};

pub fn assign_int(input: &ParseNestedMeta) -> Result<LitInt> {
    let value = input.value()?;
    value.parse()
}
