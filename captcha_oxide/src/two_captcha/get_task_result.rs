use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::captcha::{solution::Solution, Captcha};

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
    rename_all_fields = "camelCase",
    bound = "T::Solution: Deserialize<'de>"
)]
pub enum Response<'a, T>
where
    T: Captcha,
{
    Ready(Solution<'a, T>),
    Processing,

    #[serde(untagged)]
    Error {
        error_id: u8,
        error_code: Cow<'a, str>,
        error_description: Cow<'a, str>,
    },
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, collections::HashMap, net::IpAddr, str::FromStr};

    use chrono::{DateTime, Utc};
    use serde_json::{from_str, to_string};

    use crate::{
        captcha::{solution::Solution, Captcha},
        two_captcha::get_task_result::{Request, Response},
    };

    impl Captcha for () {
        type Solution = HashMap<String, String>;
        type Builder = ();

        fn get_timeout(&self) -> std::time::Duration {
            unimplemented!()
        }
    }

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
        let processing = r#"{ "errorId": 0, "status": "processing" }"#;
        let ready = r#"{
            "errorId": 0,
            "status": "ready",
            "solution": {},
            "cost": "0.00299",
            "ip": "1.2.3.4",
            "createTime": 1692863536,
            "endTime": 1692863556,
            "solveCount": 1
        }"#;
        let error = r#"{
            "errorId": 12,
            "errorCode": "ERROR_CAPTCHA_UNSOLVABLE",
            "errorDescription": "Workers could not solve the Captcha"
        }"#;

        let processing_expected = Response::Processing;
        let ready_expected = Response::Ready(Solution {
            task_id: 0,
            solution: HashMap::default(),
            cost: Cow::Borrowed("0.00299"),
            create_time: DateTime::<Utc>::from_timestamp_millis(1_692_863_536_000).unwrap(),
            end_time: DateTime::<Utc>::from_timestamp_millis(1_692_863_556_000).unwrap(),
            solve_count: 1,
            ip: IpAddr::from_str("1.2.3.4").unwrap(),
        });
        let error_expected = Response::Error {
            error_id: 12,
            error_code: Cow::Borrowed("ERROR_CAPTCHA_UNSOLVABLE"),
            error_description: Cow::Borrowed("Workers could not solve the Captcha"),
        };

        assert_eq!(
            from_str::<Response<'_, ()>>(processing).unwrap(),
            processing_expected
        );
        assert_eq!(from_str::<Response<'_, ()>>(ready).unwrap(), ready_expected);
        assert_eq!(from_str::<Response<'_, ()>>(error).unwrap(), error_expected);
    }
}
