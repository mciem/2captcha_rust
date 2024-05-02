use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::Captcha;
use builder::{
    type_state::{MissingBody, MissingComment, MissingImgInstructions},
    DrawAroundCaptchaBuilder,
};

mod builder;

/// This method can be used to bypass tasks where you need to draw
/// a line around a specific object shown on an image.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::draw_around_captcha::DrawAroundCaptcha,
/// };
///
/// let captcha = DrawAroundCaptcha::builder()
///     .body("/9j/4AAQSkZJ...OGSRF//Z")
///     .comment("Draw around an apple")
///     .build();
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DrawAroundTask")]
pub struct DrawAroundCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    body: &'a str,

    /// A comment will be shown to the workers to help them solve the captcha properly
    /// The [`DrawAroundCaptcha::comment`] property is required if
    /// [`DrawAroundCaptcha::img_instructions`] is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`DrawAroundCaptcha::img_instructions`] property is required if
    /// the [`DrawAroundCaptcha::comment`] property is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    img_instructions: Option<&'a str>,
}

impl<'a> Captcha for DrawAroundCaptcha<'a> {
    type Solution = DrawAroundCaptchaSolution;
    type Builder = DrawAroundCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions>;

    fn get_timeout(&self) -> Duration {
        Duration::from_secs(5)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DrawAroundCaptchaSolution {
    pub canvas: Box<[Box<[Point]>]>,
}
