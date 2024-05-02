use serde::Serialize;
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a reCaptcha V2
/// challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::turnstile_captcha::challenge_page::ChallengePageCaptcha,
/// };
///
/// let captcha = ChallengePageCaptcha::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .action("managed")
///     .data("80001aa1affffc21")
///     .page_data("3gAFo2l...55NDFPRFE9")
///     .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "super::solution::TurnstileCaptchaSolution<'a>",
    proxy(with_proxy = "TurnstileTask", without_proxy = "TurnstileTaskProxyless",)
)]
pub struct ChallengePageCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// Turnstile sitekey. Can be found inside the `data-sitekey` property of
    /// the Turnstile `div` element
    website_key: &'a str,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    user_agent: &'a str,

    /// The value of `action` parameter of the `turnstile.render` call
    action: &'a str,

    /// The value of `cData` parameter of the `turnstile.render` call
    data: &'a str,

    /// The value of `chlPageData` parameter of the `turnstile.render` call
    page_data: &'a str,
}
