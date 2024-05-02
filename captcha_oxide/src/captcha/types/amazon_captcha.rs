use std::borrow::Cow;

use crate::captcha::captcha;
use serde::{Deserialize, Serialize};
use url::Url;

/// Represents the data required by the 2captcha API to solve an
/// Amazon captcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::amazon_captcha::AmazonCaptcha,
/// };
///
/// let captcha = AmazonCaptcha::builder()
///     .website_url(Url::parse("https://www.example.com")?)
///     .website_key("SOME_KEY")
///     .iv("SOME_IV")
///     .context("SOME_CONTEXT")
///     .build();
///
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "AmazonCaptchaSolution<'a>",
    proxy(with_proxy = "AmazonTask", without_proxy = "AmazonTaskProxyless")
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmazonCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// Value of the `key` parameter you found on the page
    website_key: &'a str,

    /// Value of the `iv` parameter you found on the page
    iv: &'a str,

    /// Value of the `context` parameter you found on the page
    context: &'a str,

    /// The source URL of the `challenge.js` script on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_script: Option<&'a str>,

    /// The source URL of the `captcha.js` script on the page
    #[serde(skip_serializing_if = "Option::is_none")]
    captcha_script: Option<&'a str>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AmazonCaptchaSolution<'a> {
    pub captcha_voucher: Cow<'a, str>,
    pub existing_token: Cow<'a, str>,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use url::Url;

    use super::AmazonCaptcha;
    use crate::{captcha::Captcha, Result};

    #[test]
    fn builder_and_serialization() -> Result<()> {
        let task = AmazonCaptcha::builder()
            .website_url(Url::from_str("https://somewebsite.com")?)
            .website_key("somekey")
            .iv("someiv")
            .context("somecontext")
            .build();

        let json = serde_json::to_string(&task)?;

        assert_eq!(
            json,
            r#"{"websiteURL":"https://somewebsite.com/","websiteKey":"somekey","iv":"someiv","context":"somecontext","type":"AmazonTaskProxyless"}"#,
        );

        Ok(())
    }
}
