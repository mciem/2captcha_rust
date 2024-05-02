use serde::{Deserialize, Serialize};

use crate::captcha::captcha;

/// This method can be used to bypass tasks where you need to click
/// on some points of an image. \
/// It can be also used for cases where you need to calculate a distance
/// between points. \
/// \
/// For example, to bypass custom slider captchas you can instruct our worker
/// to click on a particular point of the image using the `comment` and
/// `img_instructions` parameters and then use the point coordinates to
/// calculate the distance between the slider's start and end points and move
/// the slider.
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Captcha,
///     captcha::types::coordinates_captcha::CoordinatesCaptcha,
/// };
///
/// let captcha = CoordinatesCaptcha::builder()
///     .body("/9j/4AAQSkZJRgABAQAAAQ..HIAAAAAAQwAABtbnRyUkdCIFhZ.wc5GOGSRF//Z")
///     .comment("Click the green apple")
///     .build();
/// ```
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", rename = "CoordinatesTask")]
#[captcha(crate = "crate", timeout = 5, solution = "CoordinatesCaptchaSolution")]
pub struct CoordinatesCaptcha<'a> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    body: &'a str,

    /// A comment will be shown to the workers to help them solve the captcha properly
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    img_instructions: Option<&'a str>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CoordinatesCaptchaSolution {
    pub coordinates: Box<[Point]>,
}
