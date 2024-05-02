use std::borrow::Cow;

use crate::captcha::captcha;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "TencentCaptchaSolution<'a>",
    proxy(with_proxy = "TencentTask", without_proxy = "TencentTaskProxyless",)
)]
pub struct TencentCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// The value of the `appId` parameter in the website source code.
    app_id: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TencentCaptchaSolution<'a> {
    pub app_id: Cow<'a, str>,
    pub ret: u8,
    pub ticket: Cow<'a, str>,
    pub randstr: Cow<'a, str>,
}
