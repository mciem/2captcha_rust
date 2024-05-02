use std::borrow::Cow;

use crate::captcha::captcha;

use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the data required by the 2captcha API to solve a
/// GeeTestV3 challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::geetest_captcha::v3::GeeTestV3
/// };
///
/// let captcha = GeeTestV3::builder()
///     .website_url(Url::parse("https://someurl.com/latest")?)
///     .gt("81388ea1fc187e0c335c0a8907ff2625")
///     .challenge("2e2f0f65240058b683cb6ea21c303eea6n")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "GeeTestV3Solution<'a>",
    proxy(with_proxy = "GeeTestTask", without_proxy = "GeeTestTaskProxyless",)
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// GeeTest `gt` value.
    gt: &'a str,

    /// GeeTest `challenge` value.
    challenge: &'a str,

    /// Custom GeeTest API domain, for example: `api-na.geetest.com`.
    /// Can be defined inside `initGeetest` call. Also you can check
    /// the domain used to load the scripts, the default domain is
    /// `api.geetest.com`.
    #[serde(skip_serializing_if = "Option::is_none")]
    geetest_api_server_subdomain: Option<&'a str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<&'a str>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV3Solution<'a> {
    pub challenge: Cow<'a, str>,
    pub validate: Cow<'a, str>,
    pub seccode: Cow<'a, str>,
}
