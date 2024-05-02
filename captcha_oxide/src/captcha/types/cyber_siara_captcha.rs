use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a
/// CyberSiARA captcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::cyber_siara_captcha::CyberSiARACaptcha,
/// };
///
/// let captcha = CyberSiARACaptcha::builder()
///     .website_url(Url::parse("http://some_url.com")?)
///     .slide_master_url_id("OXR2LVNvCuXykkZbB8KZIfh162sNT8S2")
///     .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "CyberSiARACaptchaSolution<'a>",
    proxy(
        with_proxy = "AntiCyberSiAraTask",
        without_proxy = "AntiCyberSiAraTaskProxyless",
    )
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CyberSiARACaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The value of the `MasterUrlId` parameter obtained from the request
    /// to the endpoint `API/CyberSiara/GetCyberSiara`.
    #[serde(rename = "SlideMasterUrlId")]
    slide_master_url_id: &'a str,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    user_agent: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CyberSiARACaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
