use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::captcha::captcha;

/// Represents the data required by the 2captcha API to solve an
/// audio based captcha challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::audio_captcha::{AudioCaptcha, Language},
/// };
///
/// let captcha = AudioCaptcha::builder()
///     .body("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
///     .language(Language::Portuguese)
///     .build();
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "AudioTask")]
#[captcha(crate = "crate", timeout = 5, solution = "AudioCaptchaSolution<'a>")]
pub struct AudioCaptcha<'a> {
    /// Base64 encoded audio file in mp3 format
    body: &'a str,

    /// The language of the audio recording.
    /// Supported languages are:
    /// * `Portuguese`
    /// * `English`
    /// * `French`
    /// * `German`
    /// * `Greek`
    /// * `Russian`
    #[serde(rename = "lang")]
    language: Language,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AudioCaptchaSolution<'a> {
    pub solution: Cow<'a, str>,
}

#[derive(Default, Serialize, Debug)]
pub enum Language {
    #[serde(rename = "en")]
    #[default]
    English,

    #[serde(rename = "pt")]
    Portuguese,

    #[serde(rename = "fr")]
    French,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "el")]
    Greek,

    #[serde(rename = "ru")]
    Russian,
}
