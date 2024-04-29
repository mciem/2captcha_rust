use proc_macro2::Ident;
use quote::format_ident;

pub trait IdentExt {
    fn to_pascal_case(&self) -> Self;
}

impl IdentExt for Ident {
    fn to_pascal_case(&self) -> Self {
        let ident = self.to_string();
        let mut output = String::with_capacity(ident.chars().filter(|&c| c != '_').count());

        let mut capitalize = true;
        for c in ident.chars() {
            if c == '_' {
                capitalize = true;
            } else if capitalize {
                output.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                output.push(c);
            }
        }

        format_ident!("{output}")
    }
}
