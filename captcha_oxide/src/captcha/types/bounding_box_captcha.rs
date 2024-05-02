use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::Captcha;

use self::builder::{
    type_state::{MissingBody, MissingComment, MissingImgInstructions},
    BoundingBoxCaptchaBuilder,
};

mod builder;

/// Can be used to solve tasks where you need to select a specific
/// object or draw a box around an object shown on an image.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::bounding_box_captcha::BoundingBoxCaptcha
/// };
///
/// let captcha = BoundingBoxCaptcha::builder()
///     .body("/9j/4AAQSkZJRgABAQAAAQ..HIAAAAAAQwAABtbnRyUkdCIFhZ.wc5GOGSRF//Z")
///     .comment("Draw a box around the car")
///     .build();
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "DrawAroundTask")]
pub struct BoundingBoxCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    body: &'a str,

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`BoundingBoxCaptcha::comment`] property is required if
    /// [`BoundingBoxCaptcha::img_instructions`] is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`BoundingBoxCaptcha::img_instructions`] property is required if
    /// the [`BoundingBoxCaptcha::comment`] property is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    img_instructions: Option<&'a str>,
}

impl<'a> Captcha for BoundingBoxCaptcha<'a> {
    type Solution = BoundingBoxCaptchaSolution;
    type Builder = BoundingBoxCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions>;

    fn get_timeout(&self) -> Duration {
        Duration::from_secs(5)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub x_min: u16,
    pub y_min: u16,
    pub x_max: u16,
    pub y_max: u16,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct BoundingBoxCaptchaSolution {
    pub bounding_boxes: Box<[Box<[BoundingBox]>]>,
}
