use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<'a> {
    pub client_key: &'a str,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum Response<'a> {
    Success { balance: f64 },
    Error { error_code: Cow<'a, str> },
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use serde_json::{from_str, to_string};

    use crate::two_captcha::get_balance::{Request, Response};

    #[test]
    fn request_serialization() {
        let request = Request {
            client_key: "API_KEY",
        };

        let expected = r#"{"clientKey":"API_KEY"}"#;

        assert_eq!(to_string(&request).unwrap(), expected);
    }

    #[test]
    fn response_deserialization() {
        let response = r#"{ "errorId": 0, "balance": 0.93958 }"#;
        let error = r#"{
            "errorId": 11,
            "errorCode": "ERROR_IP_NOT_ALLOWED",
            "errorDescription": "The request is sent from the IP that is not on the list of your trusted IPs"
        }"#;

        let expected = Response::Success { balance: 0.93958 };
        let error_expected = Response::Error {
            error_code: Cow::Borrowed("ERROR_IP_NOT_ALLOWED"),
        };

        assert_eq!(from_str::<Response<'_>>(response).unwrap(), expected);
        assert_eq!(from_str::<Response<'_>>(error).unwrap(), error_expected);
    }
}
