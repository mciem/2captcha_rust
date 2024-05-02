use std::{borrow::Cow, fmt::Debug};

use crate::captcha::{captcha, Empty};
use serde::{Deserialize, Serialize, Serializer};
use url::Url;

/// Represents the data required by the 2captcha API to solve an
/// ArkoseLabs (formerly FunCaptcha) captcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::arkose_labs_captcha::ArkoseLabsCaptcha,
/// };
///
/// let captcha = <ArkoseLabsCaptcha>::builder()
///     .website_url(Url::parse("https://www.example.com")?)
///     .website_public_key("6220FF23-9856-3A6F-9FF1-A14F88123F55")
///     .build();
///
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
///
/// The angle brackets (`<>`) around [`ArkoseLabsCaptcha`] allow the use of the
/// default type provided to the generic argument, so you don't need to
/// create a serializable unit struct if you don't plan to use the
/// `data` field
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "ArkoseLabsCaptchaSolution<'a>",
    proxy(
        with_proxy = "FunCaptchaTask",
        without_proxy = "FunCaptchaTaskProxyless"
    )
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptcha<'a, T = Empty>
where
    T: Serialize + Debug + Send + Sync,
{
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// ArkoseLabsCaptcha public key. The public key can be found in
    /// the value of the `data-pkey` parameter of the FunCaptcha `div` element,
    /// or you can find an element with `name=fc-token` and from its value cut
    /// out the key that is specified after `pk`.
    website_public_key: &'a str,

    /// Custom subdomain used to load the captcha widget, e.g.: `sample-api.arkoselabs.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    funcaptcha_api_jssubdomain: Option<&'a str>,

    /// Additional data payload object.
    /// This data will be converted to a JSON string internally
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "stringified_json"
    )]
    data: Option<T>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<&'a str>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArkoseLabsCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}

fn stringified_json<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize + Send + Sync,
    S: Serializer,
{
    serde_json::to_string(&value).unwrap().serialize(serializer)
}
