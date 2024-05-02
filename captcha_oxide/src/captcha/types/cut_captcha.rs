use std::borrow::Cow;

use crate::captcha::captcha;

use serde::{Deserialize, Serialize};
use url::Url;

/// Token-based method to bypass Cutcaptcha.
/// The token received must be set as the value attribute of
/// the `input#cap_token` element and/or passed to the callback function.
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::cut_captcha::CutCaptcha,
/// };
///
/// let captcha = CutCaptcha::builder()
///     .website_url(Url::parse("http://some_url.com")?)
///     .misery_key("a1488b66da00bf332a1488993a5443c79047e752")
///     .api_key("SAb83IIB")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "CutCaptchaSolution<'a>",
    proxy(
        with_proxy = "CutCaptchaTask",
        without_proxy = "CutCaptchaTaskProxyless",
    )
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CutCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The value of the `CUTCAPTCHA_MISERY_KEY` variable defined on the page.
    misery_key: &'a str,

    /// The value of the `data-apikey` attribute in the `iframe`'s body.
    /// Also the name of the javascript file included on the page
    api_key: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CutCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
