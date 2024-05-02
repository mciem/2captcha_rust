use serde::Serialize;
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a reCaptcha V3
/// challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::recaptcha::v3::RecaptchaV3,
/// };
///
/// let captcha = RecaptchaV3::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .min_score(0.3)
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "super::solution::RecaptchaSolution<'a>"
)]
#[derive(Debug, Serialize)]
#[serde(
    rename_all = "camelCase",
    tag = "type",
    rename = "RecaptchaV3TaskProxyless"
)]
pub struct RecaptchaV3<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// reCAPTCHA sitekey. Can be found inside `data-sitekey` property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the reCAPTHCHA API.
    /// You can also use [this script](https://gist.github.com/2captcha/2ee70fa1130e756e1693a5d4be4d8c70) to find the value
    website_key: &'a str,

    /// Required score value. Recommended values are `0.3`, `0.7` and `0.9`
    min_score: f32,

    /// Action parameter value. The value is set by website owner inside
    /// the `data-action` property of the reCAPTCHA `div` element or passed
    /// inside the options object of the `execute` method call,
    /// like `grecaptcha.execute('websiteKey', { action: 'myAction' })`
    #[serde(skip_serializing_if = "Option::is_none")]
    page_action: Option<&'a str>,

    /// Indicates the usage of the Enterprise version of reCAPTCHA.
    /// You can identify it by the `enterprise.js` script being used instead
    /// of `api.js` or by the `grecaptcha.enterprise.execute` call being used
    /// instead of `grecaptcha.execute`
    #[serde(skip_serializing_if = "Option::is_none")]
    is_enterprise: Option<bool>,

    /// Domain used to load the captcha: `google.com` or `recaptcha.net`.
    /// Default value: `google.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    api_domain: Option<&'a str>,
}
