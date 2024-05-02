use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a
/// KeyCaptcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{Captcha, captcha::types::key_captcha::KeyCaptcha};
///
/// let captcha = KeyCaptcha::builder()
///     .website_url(Url::parse("https://2captcha.com/demo/keycaptcha")?)
///     .user_id(184015_u32)
///     .session_id("8510374722aa3f99a7199d306865afb2")
///     .web_server_sign("bed1536559a1cab72ecd0e28e89c431c")
///     .web_server_sign2("104ac902450db8362ce5fc11e841ee47")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[derive(Serialize)]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "KeyCaptchaSolution<'a>",
    proxy(
        with_proxy = "KeyCaptchaTask",
        without_proxy = "KeyCaptchaTaskProxyless",
    )
)]
pub struct KeyCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The value of the `s_s_c_user_id` parameter found on page
    #[serde(rename = "s_s_c_user_id")]
    user_id: u32,

    /// The value of the `s_s_c_session_id` parameter found on page
    #[serde(rename = "s_s_c_session_id")]
    session_id: &'a str,

    /// The value of the `s_s_c_web_server_sign` parameter found on page
    #[serde(rename = "s_s_c_web_server_sign")]
    web_server_sign: &'a str,

    /// The value of the `s_s_c_web_server_sign2` parameter found on page
    #[serde(rename = "s_s_c_web_server_sign2")]
    web_server_sign2: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct KeyCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
