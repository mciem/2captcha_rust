use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    Serialize(#[from] serde_json::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    TwoCaptchaError(#[from] crate::captcha_solver::error::Error),
}
