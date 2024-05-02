use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaSolution<'a> {
    pub g_recaptcha_response: Cow<'a, str>,
    pub token: Cow<'a, str>,
}
