use std::borrow::Cow;

use crate::captcha::captcha;

use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the data required by the 2captcha API to solve a
/// CapyCaptcha puzzle
///
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::capy_captcha::CapyCaptcha
/// };
///
/// let captcha = CapyCaptcha::builder()
///     .website_url(Url::parse("http://some_url.com")?)
///     .website_key("PUZZLE_Abc1dEFghIJKLM2no34P56q7rStu8v")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "CapyCaptchaSolution<'a>",
    proxy(with_proxy = "CapyTask", without_proxy = "CapyTaskProxyless")
)]
pub struct CapyCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    website_url: Url,

    /// Capy Puzzle Captcha `captchakey`.
    website_key: &'a str,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<&'a str>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CapyCaptchaSolution<'a> {
    #[serde(rename = "captchakey")]
    pub captcha_key: Cow<'a, str>,

    #[serde(rename = "challengekey")]
    pub challenge_key: Cow<'a, str>,
    pub answer: Cow<'a, str>,

    pub resp_key: Cow<'a, str>,
}
