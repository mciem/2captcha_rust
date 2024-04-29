use std::{fmt::Debug, time::Duration};

use serde::{Deserialize, Serialize};

mod solution;
pub mod types;
pub use captcha_oxide_macros::captcha;

pub use solution::{status::Status, Solution};

pub trait Captcha: Serialize + Send + Sync {
    type Solution: for<'de> Deserialize<'de> + Debug + Send + Sync + PartialEq + Eq;
    type Builder: Default;

    fn get_timeout(&self) -> Duration;

    #[must_use]
    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}
