use std::borrow::Cow;

use crate::captcha::captcha;

use serde::{Deserialize, Serialize};

/// Represents the data required by the 2captcha API to solve a
/// text captcha challenge, where you need to answer a question
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::text_captcha::TextCaptcha,
/// };
///
/// let captcha = TextCaptcha::builder()
///     .comment("What's 2 + 2?")
///     .build();
/// ```
#[derive(Serialize)]
#[serde(tag = "type", rename = "TextCaptchaTask")]
#[captcha(crate = "crate", timeout = 5, solution = "TextCaptchaSolution<'a>")]
pub struct TextCaptcha<'a> {
    comment: &'a str,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TextCaptchaSolution<'a> {
    pub text: Cow<'a, str>,
}
