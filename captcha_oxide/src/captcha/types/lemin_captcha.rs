use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a
/// LeminCaptcha challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{Captcha, captcha::types::lemin_captcha::LeminCaptcha};
///
/// let captcha = LeminCaptcha::builder()
///     .website_url(Url::parse("https://2captcha.com/demo/lemin")?)
///     .captcha_id("CROPPED_3dfdd5c_d1872b526b794d83ba3b365eb15a200b")
///     .div_id("lemin-cropped-captcha")
///     .lemin_api_server_subdomain(Url::parse("https://api.leminnow.com")?)
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "LeminCaptchaSolution<'a>",
    proxy(with_proxy = "LeminTask", without_proxy = "LeminTaskProxyless",)
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeminCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// Lemin `captchaId` value. Unique for a website.
    captcha_id: &'a str,

    /// The `id` of the captcha's parent `div` element
    div_id: &'a str,

    /// API domain used to load the captcha scripts. Default: `https://api.leminnow.com/`
    #[serde(skip_serializing_if = "Option::is_none")]
    lemin_api_server_subdomain: Option<&'a str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<&'a str>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LeminCaptchaSolution<'a> {
    pub answer: Cow<'a, str>,

    pub challenge_id: Cow<'a, str>,
}

#[cfg(test)]
mod test {
    use std::env;

    use url::Url;

    use crate::{captcha::types::lemin_captcha::LeminCaptcha, Captcha, CaptchaSolver, Error};

    #[tokio::test]
    async fn lemin_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();
        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = LeminCaptcha::builder()
            .website_url(Url::parse("https://2captcha.com/demo/lemin")?)
            .captcha_id("CROPPED_5a29582_ca114c2f3314482c84cd32fc7d2feb63")
            .div_id("lemin-cropped-captcha")
            .lemin_api_server_subdomain("api.leminnow.com")
            .build();

        let solution = solver.solve(&captcha).await?.solution;

        assert_ne!(solution.answer, "");

        Ok(())
    }
}
