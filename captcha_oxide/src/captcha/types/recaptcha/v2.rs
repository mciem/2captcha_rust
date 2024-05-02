use serde::Serialize;
use url::Url;

use crate::{captcha::captcha, cookie::Cookies};

/// Represents the data required by the 2captcha API to solve a reCaptcha V2
/// challenge
///
/// # Example
/// ```
/// use url::Url;
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::recaptcha::v2::RecaptchaV2,
/// };
///
/// let captcha = RecaptchaV2::builder()
///     .website_url(Url::parse("http://someurl.com")?)
///     .website_key("SOME_KEY")
///     .build();
/// # Ok::<_, captcha_oxide::Error>(())
/// ```
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "super::solution::RecaptchaSolution<'a>",
    proxy(
        with_proxy = "RecaptchaV2Task",
        without_proxy = "RecaptchaV2TaskProxyless",
    )
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2<'a> {
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

    /// The value of the `data-s` parameter. Can be required to bypass the captcha on
    /// Google services
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_data_s_value: Option<&'a str>,

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

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;
    use url::Url;

    use crate::{captcha::types::recaptcha::v2::RecaptchaV2, Captcha, CaptchaSolver, Error};

    #[tokio::test]
    async fn recaptcha_v2() -> Result<(), Error> {
        dotenv().unwrap();

        let data = RecaptchaV2::builder()
            .website_url(Url::parse("https://patrickhlauke.github.io/recaptcha/")?)
            .website_key("6Ld2sf4SAAAAAKSgzs0Q13IZhY02Pyo31S2jgOB5")
            .build();

        let solver = CaptchaSolver::new(env::var("API_KEY").unwrap());

        let solution = solver.solve(&data).await?.solution.g_recaptcha_response;

        assert!(!solution.is_empty());
        Ok(())
    }
}
