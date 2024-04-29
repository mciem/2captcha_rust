use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{captcha::Captcha, language_pool::LanguagePool};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<'a, T>
where
    T: Captcha,
{
    pub client_key: &'a str,
    pub task: &'a T,
    pub soft_id: u16,
    pub language_pool: LanguagePool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<&'a Url>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum Response<'a> {
    TaskCreated { task_id: u64 },
    Error { error_code: Cow<'a, str> },
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, collections::HashMap, hash::BuildHasher};

    use crate::{
        captcha::Captcha,
        language_pool::LanguagePool,
        two_captcha::create_task::{Request, Response},
    };

    use serde_json::{from_str, to_string};

    impl<S: BuildHasher + Send + Sync> Captcha for HashMap<String, String, S> {
        type Solution = ();
        type Builder = ();

        fn get_timeout(&self) -> std::time::Duration {
            unimplemented!()
        }
    }

    #[test]
    fn request_serialization() {
        let task = HashMap::<String, String>::default();
        let request = Request {
            client_key: "API_KEY",
            task: &task,
            soft_id: 4143,
            language_pool: LanguagePool::En,
            callback_url: None,
        };

        let expected = r#"{"clientKey":"API_KEY","task":{},"softId":4143,"languagePool":"en"}"#;

        assert_eq!(to_string(&request).unwrap(), expected);
    }

    #[test]
    fn response_deserialization() {
        let success = r#"{ "errorId": 0, "taskId": 72345678901 }"#;
        let error = r#"{ "errorId": 10, "errorCode": "ERROR_ZERO_BALANCE" }"#;

        let success_expected = Response::TaskCreated {
            task_id: 72_345_678_901,
        };
        let error_expected = Response::Error {
            error_code: Cow::Borrowed("ERROR_ZERO_BALANCE"),
        };

        assert_eq!(from_str::<Response<'_>>(success).unwrap(), success_expected);
        assert_eq!(from_str::<Response<'_>>(error).unwrap(), error_expected);
    }
}
