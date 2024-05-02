pub mod type_state;
use type_state::{
    BodyProvided, CommentProvided, ImgInstructionsProvided, MissingBody, MissingComment,
    MissingImgInstructions,
};

use super::GridCaptcha;

pub struct GridCaptchaBuilder<T, U, V> {
    body: T,
    rows: Option<u8>,
    columns: Option<u8>,
    comment: U,
    img_instructions: V,
}

impl GridCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            body: MissingBody,
            rows: None,
            columns: None,
            comment: MissingComment,
            img_instructions: MissingImgInstructions,
        }
    }
}

impl Default for GridCaptchaBuilder<MissingBody, MissingComment, MissingImgInstructions> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, ImgInstructionsProvided<'a>> {
    #[must_use]
    pub const fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: Some(self.comment.0),
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, CommentProvided<'a>, MissingImgInstructions> {
    #[must_use]
    pub const fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: Some(self.comment.0),
            img_instructions: None,
        }
    }
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, MissingComment, ImgInstructionsProvided<'a>> {
    #[must_use]
    pub const fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: None,
            img_instructions: Some(self.img_instructions.0),
        }
    }
}

impl<'a> GridCaptchaBuilder<BodyProvided<'a>, MissingComment, MissingImgInstructions> {
    #[must_use]
    pub const fn build(self) -> GridCaptcha<'a> {
        GridCaptcha {
            body: self.body.0,
            rows: self.rows,
            columns: self.columns,
            comment: None,
            img_instructions: None,
        }
    }
}

impl<'a, T, U, V> GridCaptchaBuilder<T, U, V> {
    /// Image encoded into Base64 format. Data-URI format
    /// (containing `data:content/type` prefix) is also supported
    #[must_use]
    pub fn body(self, body: impl Into<&'a str>) -> GridCaptchaBuilder<BodyProvided<'a>, U, V> {
        GridCaptchaBuilder {
            body: BodyProvided(body.into()),
            rows: self.rows,
            columns: self.columns,
            comment: self.comment,
            img_instructions: self.img_instructions,
        }
    }

    /// Number of grid rows
    #[must_use]
    pub fn rows(mut self, rows: impl Into<u8>) -> Self {
        self.rows = Some(rows.into());
        self
    }

    #[must_use]
    pub const fn remove_rows(mut self) -> Self {
        self.rows = None;
        self
    }

    /// Number of grid columns
    #[must_use]
    pub fn columns(mut self, columns: impl Into<u8>) -> Self {
        self.columns = Some(columns.into());
        self
    }

    #[must_use]
    pub const fn remove_columns(mut self) -> Self {
        self.columns = None;
        self
    }

    /// A comment will be shown to workers to help them solve the captcha properly.
    /// The [`Grid::comment`] property is required if
    /// [`Grid::img_instructions`] is missing.
    #[must_use]
    pub fn comment(
        self,
        comment: impl Into<&'a str>,
    ) -> GridCaptchaBuilder<T, CommentProvided<'a>, V> {
        GridCaptchaBuilder {
            body: self.body,
            rows: self.rows,
            columns: self.columns,
            comment: CommentProvided(comment.into()),
            img_instructions: self.img_instructions,
        }
    }

    #[must_use]
    pub fn remove_comment(self) -> GridCaptchaBuilder<T, MissingComment, V> {
        GridCaptchaBuilder {
            body: self.body,
            rows: self.rows,
            columns: self.columns,
            comment: MissingComment,
            img_instructions: self.img_instructions,
        }
    }

    /// An optional image with instruction that will be shown to workers.
    /// The image must be encoded into Base64 format. Max file size: 100 kB.
    /// The [`Grid::img_instructions`] property is required if
    /// the [`Grid::comment`] property is missing.
    #[must_use]
    pub fn img_instructions(
        self,
        img_instructions: impl Into<&'a str>,
    ) -> GridCaptchaBuilder<T, U, ImgInstructionsProvided<'a>> {
        GridCaptchaBuilder {
            body: self.body,
            rows: self.rows,
            columns: self.columns,
            comment: self.comment,
            img_instructions: ImgInstructionsProvided(img_instructions.into()),
        }
    }

    #[must_use]
    pub fn remove_img_instructions(self) -> GridCaptchaBuilder<T, U, MissingImgInstructions> {
        GridCaptchaBuilder {
            body: self.body,
            rows: self.rows,
            columns: self.columns,
            comment: self.comment,
            img_instructions: MissingImgInstructions,
        }
    }
}
