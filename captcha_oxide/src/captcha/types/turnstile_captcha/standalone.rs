use serde::Serialize;
use url::Url;

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve a Turnstile
/// standalone challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::turnstile_captcha::standalone::StandaloneCaptcha,
/// };
///
/// let captcha = StandaloneCaptcha::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "super::solution::TurnstileCaptchaSolution<'a>",
    proxy(with_proxy = "TurnstileTask", without_proxy = "TurnstileTaskProxyless",)
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StandaloneCaptcha<'a> {
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    website_url: Url,

    /// Turnstile sitekey. Can be found inside the `data-sitekey` property of
    /// the Turnstile `div` element
    website_key: &'a str,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<&'a str>,
}

#[cfg(test)]
mod test {
    use std::env;

    use url::Url;

    use crate::{
        captcha::types::turnstile_captcha::standalone::StandaloneCaptcha, Captcha, CaptchaSolver,
        Error,
    };

    #[tokio::test]
    async fn turnstile_standalone_captcha() -> Result<(), Error> {
        dotenv::dotenv().unwrap();

        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let captcha = StandaloneCaptcha::builder()
            .website_url(Url::parse(
                "https://2captcha.com/demo/cloudflare-turnstile",
            )?)
            .website_key("0x4AAAAAAAVrOwQWPlm3Bnr5")
            .build();

        let solution = solver.solve(&captcha).await?.solution;

        assert!(!solution.token.is_empty(), "Empty?");

        Ok(())
    }
}
