use std::collections::HashMap;

use serde::{Serialize, Serializer};

pub struct Cookie(Box<str>, Box<str>);
pub struct Cookies(Box<[Cookie]>);

impl Cookies {
    #[must_use]
    pub fn new(cookies: impl Into<Box<[Cookie]>>) -> Self {
        Self(cookies.into())
    }
}

impl ToString for Cookies {
    fn to_string(&self) -> String {
        let mut output = String::new();

        let mut iter = self.0.iter().peekable();
        loop {
            let Some(Cookie(key, val)) = iter.next() else {
                return output;
            };

            output.push_str(key);
            output.push('=');
            output.push_str(val);

            if iter.peek().is_some() {
                output.push(';');
            }
        }
    }
}

impl Cookie {
    #[must_use]
    pub fn new(key: impl Into<Box<str>>, value: impl Into<Box<str>>) -> Self {
        Self(key.into(), value.into())
    }
}

impl Serialize for Cookies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<K, V> From<HashMap<K, V>> for Cookies
where
    K: Into<Box<str>>,
    V: Into<Box<str>>,
{
    fn from(value: HashMap<K, V>) -> Self {
        Self(
            value
                .into_iter()
                .map(|(key, value)| Cookie(key.into(), value.into()))
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "This test just ensures the `into` call compiles"]
    fn test_into() {
        let dict = HashMap::<&str, &str>::new();
        let _cookies: Cookies = dict.into();
    }
}
