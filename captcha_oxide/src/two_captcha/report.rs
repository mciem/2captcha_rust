use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<'a> {
    pub client_key: &'a str,
    pub task_id: u64,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(
    tag = "status",
    rename_all = "lowercase",
    rename_all_fields = "camelCase"
)]
pub enum Response<'a> {
    Success,

    #[serde(untagged)]
    Error {
        error_code: Cow<'a, str>,
    },
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use serde_json::{from_str, to_string};

    use crate::two_captcha::report::{Request, Response};

    #[test]
    fn request_serialization() {
        let request = Request {
            client_key: "API_KEY",
            task_id: 0,
        };

        let expected = r#"{"clientKey":"API_KEY","taskId":0}"#;

        assert_eq!(to_string(&request).unwrap(), expected);
    }

    #[test]
    fn response_deserialization() {
        let response = r#"{ "status": "success" }"#;
        let error = r#"{
            "errorId": 16,
            "errorCode": "ERROR_NO_SUCH_CAPCHA_ID",
            "errorDescription": "You've provided incorrect captcha ID in the request"
        }"#;

        let expected = Response::Success;
        let error_expected = Response::Error {
            error_code: Cow::Borrowed("ERROR_NO_SUCH_CAPCHA_ID"),
        };

        assert_eq!(from_str::<Response<'_>>(response).unwrap(), expected);
        assert_eq!(from_str::<Response<'_>>(error).unwrap(), error_expected);
    }
}
