use std::borrow::Cow;
use url::Url;

use serde::{Deserialize, Serialize};

use crate::captcha::{captcha, Empty};

/// Represents the data required by the 2captcha API to solve a
/// HCaptcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{Captcha, captcha::types::h_captcha::HCaptcha};
///
/// let captcha = <HCaptcha>::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
///
/// The angle brackets (`<>`) around [`HCaptcha`] allow the use of the
/// default type provided to the generic argument, so you don't need to
/// create a serializable unit struct if you don't plan to use the
/// `enterprise_payload` field
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "HCaptchaSolution<'a>",
    proxy(with_proxy = "HCaptchaTask", without_proxy = "HCaptchaTaskProxyless",)
)]
pub struct HCaptcha<'a, T = Empty>
where
    T: serde::Serialize + Send + Sync,
{
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    website_key: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_invisible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enterprise_payload: Option<T>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
    pub resp_key: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
    pub g_recaptcha_response: Cow<'a, str>,
}
