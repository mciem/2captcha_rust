#![allow(clippy::module_name_repetitions)]

use url::Url;

use crate::{language_pool::LanguagePool, CaptchaSolver};

pub struct MissingApiKey;
pub struct ApiKey(Box<str>);

pub struct CaptchaSolverBuilder<T> {
    api_key: T,
    language_pool: LanguagePool,
    callback_url: Option<Url>,
}

impl CaptchaSolverBuilder<ApiKey> {
    #[must_use]
    pub fn build(self) -> CaptchaSolver {
        CaptchaSolver {
            api_key: self.api_key.0,
            language_pool: self.language_pool,
            callback_url: self.callback_url,
        }
    }
}

impl CaptchaSolverBuilder<MissingApiKey> {
    #[must_use = "A builder type must have its `build` method called to build the target type"]
    pub const fn new() -> Self {
        Self {
            api_key: MissingApiKey,
            language_pool: LanguagePool::En,
            callback_url: None,
        }
    }

    #[must_use = "A builder type must have its `build` method called to build the target type"]
    pub fn api_key<T>(self, api_key: T) -> CaptchaSolverBuilder<ApiKey>
    where
        T: Into<Box<str>>,
    {
        CaptchaSolverBuilder {
            api_key: ApiKey(api_key.into()),
            language_pool: self.language_pool,
            callback_url: self.callback_url,
        }
    }
}

impl<T> CaptchaSolverBuilder<T> {
    #[must_use = "A builder type must have its `build` method called to build the target type"]
    pub const fn language_pool(mut self, language_pool: LanguagePool) -> Self {
        self.language_pool = language_pool;
        self
    }

    #[must_use = "A builder type must have its `build` method called to build the target type"]
    pub fn callback_url(mut self, callback_url: Url) -> Self {
        self.callback_url = Some(callback_url);
        self
    }

    #[must_use = "A builder type must have its `build` method called to build the target type"]
    pub fn remove_callback_url(mut self) -> Self {
        self.callback_url = None;
        self
    }
}

impl Default for CaptchaSolverBuilder<MissingApiKey> {
    fn default() -> Self {
        Self::new()
    }
}
