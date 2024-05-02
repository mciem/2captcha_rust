use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a
/// FriendlyCaptcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{Captcha, captcha::types::friendly_captcha::FriendlyCaptcha};
///
/// let captcha = FriendlyCaptcha::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "FriendlyCaptchaSolution<'a>",
    proxy(
        with_proxy = "FriendlyCaptchaTask",
        without_proxy = "FriendlyCaptchaTaskProxyless",
    )
)]
pub struct FriendlyCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    website_key: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FriendlyCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
