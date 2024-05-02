use std::time::Duration;

use lazy_static::lazy_static;
use reqwest::Client;
use tokio::time::sleep;
use url::Url;

use crate::{
    captcha::{
        solution::{Solution, Status},
        Captcha,
    },
    language_pool::LanguagePool,
    two_captcha::{create_task, get_balance, get_task_result, report},
    Result, SOFT_ID,
};

use self::builder::{CaptchaSolverBuilder, MissingApiKey};

mod builder;
pub mod error;

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref API_URL: Url = Url::parse("https://api.2captcha.com/").unwrap();
    static ref CREATE_TASK_URL: Url = API_URL.join("/createTask").unwrap();
    static ref GET_TASK_RESULT_URL: Url = API_URL.join("/getTaskResult").unwrap();
    static ref GET_BALANCE_URL: Url = API_URL.join("/getBalance").unwrap();
    static ref REPORT_CORRECT_URL: Url = API_URL.join("/reportCorrect").unwrap();
    static ref REPORT_INCORRECT_URL: Url = API_URL.join("/reportIncorrect").unwrap();
}

/// This struct is responsible for handling all of your interactions with the
/// 2captcha API. Use the [`CaptchaSolver::new`] method to instantiate it with
/// a given API key or the [`CaptchaSolver::builder`] method to configure other
/// settings
pub struct CaptchaSolver {
    api_key: Box<str>,
    language_pool: LanguagePool,

    #[cfg(feature = "callback")]
    callback_url: Option<Url>,
}

impl CaptchaSolver {
    /// Returns a new instance of [`CaptchaSolver`] with the given API key
    #[must_use]
    pub fn new<T>(api_key: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self {
            api_key: api_key.into(),
            language_pool: LanguagePool::En,

            #[cfg(feature = "callback")]
            callback_url: None,
        }
    }

    /// Returns a new instance of `CaptchaSolverBuilder`, which allows you to configure
    /// your [`CaptchaSolver`] and guarantees an API key is provided
    #[must_use]
    pub const fn builder() -> CaptchaSolverBuilder<MissingApiKey> {
        CaptchaSolverBuilder::new()
    }

    #[cfg(not(feature = "callback"))]
    /// Sends a request to the 2captcha API to solve the given puzzle
    ///
    /// # Errors
    /// This function can error if the HTTP request is not sent successfully,
    /// if the response cannot be parsed or if the 2captcha API returns an error
    pub async fn solve<'a, T>(&self, task: &T) -> Result<Solution<'a, T>>
    where
        T: Captcha,
    {
        let task_id = self.create_task(task).await?;

        sleep(task.get_timeout()).await;

        self.get_task_result(task_id).await
    }

    #[cfg(feature = "callback")]
    /// Sends a request to the 2captcha API to solve the given puzzle
    ///
    /// # Errors
    /// This function can error if the HTTP request is not sent successfully,
    /// if the response cannot be parsed or if the 2captcha API returns an error
    ///
    /// # Option
    /// This function will only ever return `Ok(None)` if the `CaptchaSolver::callback_url`
    /// field is set. Otherwise, it is safe to `unwrap` the [`Option`] within the [`Result`]
    pub async fn solve<'a, T>(&self, task: &T) -> Result<Option<Solution<'a, T>>>
    where
        T: Captcha,
    {
        let task_id = self.create_task(task).await?;

        if self.callback_url.is_some() {
            return Ok(None);
        }

        sleep(task.get_timeout()).await;

        self.get_task_result(task_id).await.map(Some)
    }

    async fn create_task<T>(&self, task: &T) -> Result<u64>
    where
        T: Captcha,
    {
        let request = create_task::Request {
            client_key: &self.api_key,
            task,
            soft_id: SOFT_ID,
            #[cfg(feature = "callback")]
            callback_url: self.callback_url.as_ref(),
            language_pool: self.language_pool,
        };

        let response = CLIENT
            .post(CREATE_TASK_URL.as_ref())
            .json(&request)
            .send()
            .await?
            .json::<create_task::Response>()
            .await?;

        Ok(error::Result::from(response)?)
    }

    async fn get_task_result<'a, T>(&self, task_id: u64) -> Result<Solution<'a, T>>
    where
        T: Captcha,
    {
        let request = get_task_result::Request {
            client_key: &self.api_key,
            task_id,
        };

        loop {
            let response = CLIENT
                .post(GET_TASK_RESULT_URL.as_ref())
                .json(&request)
                .send()
                .await?
                .json::<get_task_result::Response<'a, T>>()
                .await?;

            let captcha_solution = error::Result::<_>::from(response)?;

            if let Some(mut captcha_solution) = captcha_solution {
                captcha_solution.task_id = task_id;
                return Ok(captcha_solution);
            }

            sleep(Duration::from_secs(5)).await;
        }
    }

    /// Sends a request to the 2captcha API to return your current balance
    ///
    /// # Errors
    /// This function can error if the HTTP request is not sent successfully,
    /// if the response cannot be parsed or if the 2captcha API returns an error
    pub async fn get_balance(&self) -> Result<f64> {
        let request = get_balance::Request {
            client_key: &self.api_key,
        };

        let response = CLIENT
            .post(GET_BALANCE_URL.as_ref())
            .json(&request)
            .send()
            .await?
            .json::<get_balance::Response>()
            .await?;

        error::Result::<_>::from(response).map_err(Into::into)
    }

    /// Sends a request to the 2captcha API infroming whether or not the solution
    /// you received was valid
    ///
    /// # Errors
    /// This function can error if the HTTP request is not sent successfully,
    /// if the response cannot be parsed or if the 2captcha API returns an error
    pub async fn report<'a, T>(&self, solution: Solution<'a, T>, status: Status) -> Result<()>
    where
        T: Captcha,
    {
        let request = report::Request {
            client_key: &self.api_key,
            task_id: solution.task_id,
        };

        let response = CLIENT
            .post(match status {
                Status::Correct => REPORT_CORRECT_URL.as_ref(),
                Status::Incorrect => REPORT_INCORRECT_URL.as_ref(),
            })
            .json(&request)
            .send()
            .await?
            .json::<report::Response>()
            .await?;

        error::Result::<_>::from(response).map_err(Into::into)
    }
}
