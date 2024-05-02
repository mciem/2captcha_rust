pub mod type_state;
use type_state::{
    BodyProvided, CommentProvided, ImgInstructionsProvided, MissingBody, MissingComment,
    MissingImgInstructions,
};

use super::BoundingBoxCaptcha;

pub struct BoundingBoxCaptchaBuilder<T, U, V> {
    body: T,
    comment: U,
    img_instructions: V,
}

impl BoundingBoxCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            body: MissingBody,
            comment: MissingComment,
            img_instructions: MissingImgInstructions,
        }
    }
}

impl Default for BoundingBoxCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a>
    BoundingBoxCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsProvided<'a>>
{
    #[must_use]
    pub const fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> BoundingBoxCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, MissingImgInstructions> {
    #[must_use]
    pub const fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: None,
        }
    }
}

impl<'a> BoundingBoxCaptchaBuilder<BodyProvided<'a>, MissingComment, ImgInstructionsProvided<'a>> {
    #[must_use]
    pub const fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: None,
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> BoundingBoxCaptchaBuilder<BodyProvided<'a>, MissingComment, MissingImgInstructions> {
    #[must_use]
    pub const fn build(self) -> BoundingBoxCaptcha<'a> {
        BoundingBoxCaptcha {
            body: self.body.0,
            comment: None,
            img_instructions: None,
        }
    }
}

impl<'a, T, U, V> BoundingBoxCaptchaBuilder<T, U, V> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    #[must_use]
    pub fn body(
        self,
        body: impl Into<&'a str>,
    ) -> BoundingBoxCaptchaBuilder<BodyProvided<'a>, U, V> {
        BoundingBoxCaptchaBuilder {
            body: BodyProvided(body.into()),
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`BoundingBoxCaptcha::comment`] property is required if
    /// [`BoundingBoxCaptcha::img_instructions`] is missing.
    #[must_use]
    pub fn comment(
        self,
        comment: impl Into<&'a str>,
    ) -> BoundingBoxCaptchaBuilder<T, CommentProvided<'a>, V> {
        BoundingBoxCaptchaBuilder {
            body: self.body,
            comment: CommentProvided(comment.into()),
            img_instructions: self.img_instructions,
        }
    }

    #[must_use]
    pub fn remove_comment(self) -> BoundingBoxCaptchaBuilder<T, MissingComment, V> {
        BoundingBoxCaptchaBuilder {
            body: self.body,
            comment: MissingComment,
            img_instructions: self.img_instructions,
        }
    }

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`BoundingBoxCaptcha::img_instructions`] property is required if
    /// the [`BoundingBoxCaptcha::comment`] property is missing.
    #[must_use]
    pub fn img_instructions(
        self,
        img_instructions: impl Into<&'a str>,
    ) -> BoundingBoxCaptchaBuilder<T, U, ImgInstructionsProvided<'a>> {
        BoundingBoxCaptchaBuilder {
            body: self.body,
            comment: self.comment,
            img_instructions: ImgInstructionsProvided(img_instructions.into()),
        }
    }

    #[must_use]
    pub fn remove_img_instructions(
        self,
    ) -> BoundingBoxCaptchaBuilder<T, U, MissingImgInstructions> {
        BoundingBoxCaptchaBuilder {
            body: self.body,
            comment: self.comment,
            img_instructions: MissingImgInstructions,
        }
    }
}
