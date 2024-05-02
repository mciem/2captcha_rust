use thiserror::Error;

use crate::{
    captcha::{solution::Solution, Captcha},
    two_captcha::{create_task, get_balance, get_task_result, report},
};

pub type Result<T> = core::result::Result<T, Error>;

/// Represents all the errors that can be returned by the 2captcha API
#[derive(Error, Debug)]
pub enum Error {
    #[error("Your API key is incorrect. Make sure you set the key correctly and copied it from the dashboard in Customer or Developer mode")]
    InvalidApiKey,

    #[error("Your bid is too low for the captcha you submit or the queue of your captchas is loo long and the API temporarily does not accept more captchas from you")]
    NoSlotAvailable,

    #[error("Image size is smaller than 100 bytes")]
    ImageTooSmall,

    #[error("Image is larger than 100kB or bigger than 600px on any side")]
    ImageTooBig,

    #[error("You don't have funds on your account")]
    ZeroBalance,

    #[error("The request was sent from an IP that is not in your list of trusted IPs")]
    IpNotAllowed,

    #[error("Unable to solve captcha - three workers were unable solve it. The captcha price is automatically returned to your balance")]
    UnsolvableCaptcha,

    #[error("The error is returned when the 100% accuracy feature is enabled. The error means that the max numbers of attempts was reached but the min number of matches was not found")]
    BadDuplicates,

    #[error("Request made to an API route that does not exist")]
    NoSuchMethod,

    #[error("The image can not be processed due to an incorrect format or size, or the image is corrupted. Please check the image in your request payload")]
    UnsupportedImageType,

    #[error("You've provided an incorrect captcha ID in the request")]
    CaptchaIdNotFound,

    #[error("Your IP address is banned due to improper use of the API")]
    IpBlocked,

    #[error("The `task` property is missing in your call to `createTask`")]
    TaskNotProvided,

    #[error("The `task` property in your call to `createTask` contains a type of task that is not supported the API")]
    TaskNotSupported,

    #[error("The `sitekey` value provided in your request is not valid")]
    InvalidSiteKey,

    #[error("Your API access was blocked for improper use of the API. Please contact support to resolve the issue")]
    AccountSuspended,

    #[error("Unable to establish connection through the proxy")]
    BadProxy,

    #[error("Could not connect to proxy")]
    ProxyConnectionFailed,

    #[error("The required captcha parameters in your request are missing or incorrect")]
    BadParameters,

    #[error("The error is returned in cases when `imgInstructions` contains an unsupported file type, corrupted file or the size of the image is over the limits. The limits are described in the corresponding task type specification.")]
    BadImageInstructions,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        match value {
            "ERROR_KEY_DOES_NOT_EXIST" => Self::InvalidApiKey,
            "ERROR_NO_SLOT_AVAILABLE" => Self::NoSlotAvailable,
            "ERROR_ZERO_CAPTCHA_FILESIZE" => Self::ImageTooSmall,
            "ERROR_TOO_BIG_CAPTCHA_FILESIZE" => Self::ImageTooBig,
            "ERROR_ZERO_BALANCE" => Self::ZeroBalance,
            "ERROR_IP_NOT_ALLOWED" => Self::IpNotAllowed,
            "ERROR_CAPTCHA_UNSOLVABLE" => Self::UnsolvableCaptcha,
            "ERROR_BAD_DUPLICATES" => Self::BadDuplicates,
            "ERROR_NO_SUCH_METHOD" => Self::NoSuchMethod,
            "ERROR_IMAGE_TYPE_NOT_SUPPORTED" => Self::UnsupportedImageType,
            "ERROR_NO_SUCH_CAPCHA_ID" => Self::CaptchaIdNotFound,
            "ERROR_IP_BLOCKED" => Self::IpBlocked,
            "ERROR_TASK_ABSENT" => Self::TaskNotProvided,
            "ERROR_TASK_NOT_SUPPORTED" => Self::TaskNotSupported,
            "ERROR_RECAPTCHA_INVALID_SITEKEY" => Self::InvalidSiteKey,
            "ERROR_ACCOUNT_SUSPENDED" => Self::AccountSuspended,
            "ERROR_BAD_PROXY" => Self::BadProxy,
            "ERROR_PROXY_CONNECTION_FAILED" | "ERR_PROXY_CONNECTION_FAILED" => {
                Self::ProxyConnectionFailed
            }
            "ERROR_BAD_PARAMETERS" => Self::BadParameters,
            "ERROR_BAD_IMGINSTRUCTIONS" => Self::BadImageInstructions,
            x => unreachable!("Unreachable 2captcha error: {}", x),
        }
    }
}

impl<'a> From<create_task::Response<'a>> for Result<u64> {
    fn from(val: create_task::Response<'a>) -> Self {
        use create_task::Response;
        match val {
            Response::TaskCreated { task_id } => Ok(task_id),
            Response::Error { error_code, .. } => {
                let error_code = error_code.as_ref();

                Err(error_code.into())
            }
        }
    }
}

impl<'a, T> From<get_task_result::Response<'a, T>> for Result<Option<Solution<'a, T>>>
where
    T: Captcha,
{
    fn from(val: get_task_result::Response<'a, T>) -> Self {
        use get_task_result::Response;
        match val {
            Response::Ready(solution) => Ok(Some(solution)),
            Response::Processing => Ok(None),
            Response::Error { error_code, .. } => {
                let error_code = error_code.as_ref();

                Err(error_code.into())
            }
        }
    }
}

impl<'a> From<get_balance::Response<'a>> for Result<f64> {
    fn from(value: get_balance::Response) -> Self {
        use get_balance::Response;
        match value {
            Response::Success { balance } => Ok(balance),
            Response::Error { error_code } => Err(error_code.as_ref().into()),
        }
    }
}

impl<'a> From<report::Response<'a>> for Result<()> {
    fn from(value: report::Response<'a>) -> Self {
        use report::Response;
        match value {
            Response::Success => Ok(()),
            Response::Error { error_code } => Err(error_code.as_ref().into()),
        }
    }
}
