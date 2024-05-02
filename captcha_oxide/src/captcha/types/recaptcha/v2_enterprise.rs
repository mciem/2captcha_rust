use std::fmt::Debug;

use serde::Serialize;
use url::Url;

use crate::{
    captcha::{captcha, Empty},
    cookie::Cookies,
};

/// Represents the data required by the 2captcha API to solve a
/// reCaptcha V2 Enterprise challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::recaptcha::v2_enterprise::RecaptchaV2Enterprise
/// };
///
/// let captcha = <RecaptchaV2Enterprise>::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
///
/// The angle brackets (`<>`) around [`RecaptchaV2Enterprise`] allow the
/// use of the default type provided to the generic argument, so you don't
/// need to create a serializable unit struct if you don't plan to use the
/// `enterprise_payload` field
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "super::solution::RecaptchaSolution<'a>",
    proxy(
        with_proxy = "RecaptchaV2EnterpriseTask",
        without_proxy = "RecaptchaV2EnterpriseTaskProxyless",
    )
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2Enterprise<'a, T = Empty>
where
    T: Serialize + Debug + Send + Sync,
{
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// reCAPTCHA sitekey. Can be found inside `data-sitekey` property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the reCAPTHCHA API.
    /// You can also use [this script](https://gist.github.com/2captcha/2ee70fa1130e756e1693a5d4be4d8c70)
    /// to find the value
    website_key: &'a str,

    /// Additional parameters passed to `grecaptcha.enterprise.render` call. For example,
    /// there can be an object containing and `s` value
    #[serde(skip_serializing_if = "Option::is_none")]
    enterprise_payload: Option<T>,

    /// Indicates the use of the invisible version of reCAPTCHA - a case when you
    /// don't see the checkbox, but the challenge appears. Mostly used with a
    /// callback function
    #[serde(skip_serializing_if = "Option::is_none")]
    is_invisible: Option<bool>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<&'a str>,

    /// Your cookies will be set in a browser of our worker. Suitable
    /// for captcha on Google services.
    ///
    /// May be passed in as an iterable (array, slice or [Vec]) of
    /// [`crate::cookie::Cookie`] or [`(impl ToString, impl ToString)`]
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Cookies>,

    /// Domain used to load the captcha: `google.com` or `recaptcha.net`.
    /// Default value: `google.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    api_domain: Option<&'a str>,
}
