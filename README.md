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

License: MIT OR Apache-2.0
