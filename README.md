# captcha_oxide

A high level async library that allows you to use the 2captcha API
to solve various types of captcha puzzles

## Example
```rust
use captcha_oxide::{
  CaptchaSolver,
  captcha::types::recaptcha::v3::RecaptchaV3,
  Captcha,
};

use url::Url;

async fn example() -> captcha_oxide::Result<()> {
  let solver = CaptchaSolver::new("YOUR TWOCAPTCHA API KEY");

  let args = RecaptchaV3::builder()
    .website_url(Url::parse("https://someurl.com")?)
    .website_key("SITE_KEY")
    .min_score(0.3)
    .build();

  let solution = solver
    .solve(args)
    .await?
    .solution
    .g_recaptcha_response;

  assert!(!solution.is_empty());

  Ok(())
}
```

## Should I migrate from v5.2.0?
Unless you need `TencentCaptcha` or `AtbCaptcha`, there is no
need to migrate if you have an existing project using version
`5.2.0`.

Version `6.0.0` is a restructure of the API and the macros
aimed at making them more convinient to maintain, especially
as the previous macro system was cumbersome and had very poor
syntax. If you are creating a new project, we do recommend using
version 6.

## MSRV
The Minimum Supported Rust Version is 1.70.0

License: MIT OR Apache-2.0
