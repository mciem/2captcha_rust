use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a
/// MtCaptcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{Captcha, captcha::types::mt_captcha::MtCaptcha};
///
/// let captcha = MtCaptcha::builder()
///     .website_url(Url::parse("https://some_url.com")?)
///     .website_key("SOME_SITE_KEY")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "MtCaptchaSolution<'a>",
    proxy(with_proxy = "MtCaptchaTask", without_proxy = "MtCaptchaTaskProxyless",)
)]
pub struct MtCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The MTCaptcha `sitekey` value found in the page code.
    website_key: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct MtCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
