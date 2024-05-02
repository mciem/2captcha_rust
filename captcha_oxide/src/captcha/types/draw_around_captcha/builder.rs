pub mod type_state;
use type_state::{
    BodyProvided, CommentProvided, ImgInstructionsProvided, MissingBody, MissingComment,
    MissingImgInstructions,
};

use super::DrawAroundCaptcha;

pub struct DrawAroundCaptchaBuilder<T, U, V> {
    body: T,
    comment: U,
    img_instructions: V,
}

impl DrawAroundCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            body: MissingBody,
            comment: MissingComment,
            img_instructions: MissingImgInstructions,
        }
    }
}

impl Default for DrawAroundCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a>
    DrawAroundCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsProvided<'a>>
{
    #[must_use]
    pub const fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> DrawAroundCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, MissingImgInstructions> {
    #[must_use]
    pub const fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: Some(self.comment.0),
            img_instructions: None,
        }
    }
}

impl<'a> DrawAroundCaptchaBuilder<BodyProvided<'a>, MissingComment, ImgInstructionsProvided<'a>> {
    #[must_use]
    pub const fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: None,
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> DrawAroundCaptchaBuilder<BodyProvided<'a>, MissingComment, MissingImgInstructions> {
    #[must_use]
    pub const fn build(self) -> DrawAroundCaptcha<'a> {
        DrawAroundCaptcha {
            body: self.body.0,
            comment: None,
            img_instructions: None,
        }
    }
}

impl<'a, T, U, V> DrawAroundCaptchaBuilder<T, U, V> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    #[must_use]
    pub fn body(
        self,
        body: impl Into<&'a str>,
    ) -> DrawAroundCaptchaBuilder<BodyProvided<'a>, U, V> {
        DrawAroundCaptchaBuilder {
            body: BodyProvided(body.into()),
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`DrawAround::comment`] property is required if
    /// [`DrawAround::img_instructions`] is missing.
    #[must_use]
    pub fn comment(
        self,
        comment: impl Into<&'a str>,
    ) -> DrawAroundCaptchaBuilder<T, CommentProvided<'a>, V> {
        DrawAroundCaptchaBuilder {
            body: self.body,
            comment: CommentProvided(comment.into()),
            img_instructions: self.img_instructions,
        }
    }

    #[must_use]
    pub fn remove_comment(self) -> DrawAroundCaptchaBuilder<T, MissingComment, V> {
        DrawAroundCaptchaBuilder {
            body: self.body,
            comment: MissingComment,
            img_instructions: self.img_instructions,
        }
    }

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`DrawAround::img_instructions`] property is required if
    /// the [`DrawAround::comment`] property is missing.
    #[must_use]
    pub fn img_instructions(
        self,
        img_instructions: impl Into<&'a str>,
    ) -> DrawAroundCaptchaBuilder<T, U, ImgInstructionsProvided<'a>> {
        DrawAroundCaptchaBuilder {
            body: self.body,
            comment: self.comment,
            img_instructions: ImgInstructionsProvided(img_instructions.into()),
        }
    }

    #[must_use]
    pub fn remove_img_instructions(self) -> DrawAroundCaptchaBuilder<T, U, MissingImgInstructions> {
        DrawAroundCaptchaBuilder {
            body: self.body,
            comment: self.comment,
            img_instructions: MissingImgInstructions,
        }
    }
}
