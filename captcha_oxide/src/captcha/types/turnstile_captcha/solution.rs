use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TurnstileCaptchaSolution<'a> {
    pub token: Cow<'a, str>,
    pub user_agent: Cow<'a, str>,
}
