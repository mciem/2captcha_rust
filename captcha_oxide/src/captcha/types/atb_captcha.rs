use std::borrow::Cow;

use crate::captcha::captcha;
use serde::{Deserialize, Serialize};
use url::Url;

#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "AtbCaptchaSolution<'a>",
    proxy(
        with_proxy = "AtbCaptchaTask",
        without_proxy = "AtbCaptchaTaskProxyless",
    )
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AtbCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The value of the `appId` parameter in the website source code.
    app_id: &'a str,

    /// The value of the `apiServer` parameter in the website source code.
    api_server: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AtbCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
