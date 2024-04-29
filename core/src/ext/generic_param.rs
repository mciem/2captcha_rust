use syn::{parse_quote, ConstParam, GenericArgument, GenericParam, LifetimeParam, TypeParam};

pub trait GenericParamExt {
    fn as_generic_arg(&self) -> GenericArgument;
}

impl GenericParamExt for GenericParam {
    fn as_generic_arg(&self) -> GenericArgument {
        match self {
            Self::Lifetime(LifetimeParam { ref lifetime, .. }) => {
                GenericArgument::Lifetime(lifetime.clone())
            }
            Self::Type(TypeParam { ref ident, .. }) => GenericArgument::Type(parse_quote!(#ident)),
            Self::Const(ConstParam { ref ident, .. }) => {
                GenericArgument::Const(parse_quote!(#ident))
            }
        }
    }
}
