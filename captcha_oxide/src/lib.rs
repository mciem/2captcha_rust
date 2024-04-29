//! # `captcha_oxide`
//!
//! A high level async library that allows you to use the 2captcha API
//! to solve various types of captcha puzzles
//!
//! # Example

#![deny(clippy::pedantic, clippy::nursery)]
#![forbid(unsafe_code)]

pub(crate) const SOFT_ID: u16 = 4143;

pub mod captcha;
mod captcha_solver;
pub mod cookie;
mod language_pool;
mod prelude;
pub mod proxy;
mod two_captcha;

pub use captcha_solver::CaptchaSolver;
pub use prelude::{Error, Result};
