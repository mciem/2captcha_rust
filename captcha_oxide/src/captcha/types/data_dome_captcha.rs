use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{captcha::captcha, proxy::Proxy};

/// Token-based method for automated solving of DataDome. \
/// To solve the DataDome captcha, you **must** use a proxy. \
/// \
/// ## Attention
/// You need to check the value of the parameter `t` in `captcha_url`,
/// the value of `t` must be equal to `fe`. \
/// If `t=bv`, it means that your ip is banned by the captcha and you
/// need to change the ip address. \
/// \
/// ## Attention
/// You need to monitor the quality of the proxy used. If your proxy
/// is blocked by DataDome you will receive the following solving errors:
/// * [`crate::Error::TwoCaptchaError(crate::captcha_solver::error::Error::ProxyConnectionFailed)`]
/// * [`crate::Error::TwoCaptchaError(crate::captcha_solver::error::Error::UnsolvableCaptcha)`] \
/// In which case you need to change the proxy server used.
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::data_dome_captcha::DataDomeCaptcha,
///     proxy::{Proxy, Address, Kind}
/// };
///
/// let captcha = DataDomeCaptcha::builder()
///     .website_url(Url::parse("https://some_url.com/")?)
///     .captcha_url(Url::parse("https://other_url.com/")?)
///     .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
///     .proxy(Proxy {
///         kind: Kind::Http,
///         address: Address::HostName("some.proxy.com".into()),
///         port: 1234,
///         login: None,
///         password: None,
///     })
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "DataDomeCaptchaSolution<'a>"
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DataDomeSliderTask")]
pub struct DataDomeCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The value of the `src` parameter for the `iframe` element containing
    /// the captcha on the page.
    captcha_url: Url,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    user_agent: &'a str,

    /// Proxy connection data
    #[serde(flatten)]
    proxy: Proxy<'a>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DataDomeCaptchaSolution<'a> {
    pub cookie: Cow<'a, str>,
}
