use serde::{Deserialize, Serialize};

use crate::captcha::captcha;

/// This method is used to solve captchas where you need to rotate an object
/// to place it properly. Returns the required rotation angle.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::rotate_captcha::RotateCaptcha,
/// };
///
/// let captcha = RotateCaptcha::builder()
///     .body("R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==")
///     .comment("Position the image properly")
///     .angle(60_u16)
///     .build();
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "RotateTask")]
#[captcha(crate = "crate", timeout = 5, solution = "RotateCaptchaSolution")]
pub struct RotateCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    body: &'a str,

    /// One step rotation angle. You can count how many steps are required
    /// to rotate the image 360 degrees and then divide 360 by this count
    /// to get the angle value
    #[serde(skip_serializing_if = "Option::is_none")]
    angle: Option<u16>,

    /// A comment will be shown to the workers to help them solve the captcha properly
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    img_instructions: Option<&'a str>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct RotateCaptchaSolution {
    pub rotate: u16,
}
