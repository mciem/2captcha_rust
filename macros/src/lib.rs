use captcha_oxide_core::macros;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn captcha(args: TokenStream, item: TokenStream) -> TokenStream {
    macros::expansion::attr::captcha::captcha(args.into(), &item.into()).into()
}
