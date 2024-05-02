use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::Captcha;

mod builder;

use builder::{
    type_state::{MissingBody, MissingComment, MissingImgInstructions},
    GridCaptchaBuilder,
};

/// This method can be used to bypass tasks where a grid is applied to an
/// image and you need to click on grid tiles, like reCAPTCHA or hCaptcha images.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::grid_captcha::GridCaptcha,
/// };
///
/// let captcha = GridCaptcha::builder()
///     .body("/9j/4AAQSkZJ...OGSRF//Z")
///     .comment("Select all vehicles")
///     .rows(3)
///     .columns(3)
///     .build();
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "GridTask")]
pub struct GridCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    body: &'a str,

    /// Number of grid rows
    rows: Option<u8>,

    /// Number of grid columns
    columns: Option<u8>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    comment: Option<&'a str>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    img_instructions: Option<&'a str>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct GridCaptchaSolution {
    pub click: Box<[u8]>,
}

impl<'a> Captcha for GridCaptcha<'a> {
    type Solution = GridCaptchaSolution;
    type Builder = GridCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions>;

    fn get_timeout(&self) -> Duration {
        Duration::from_secs(5)
    }
}
