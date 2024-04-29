use std::borrow::Cow;

use serde::{Serialize, Serializer};

mod address;
mod kind;

pub use address::Address;
pub use kind::Kind;

#[derive(Serialize, Debug)]
pub struct Proxy<'a> {
    #[serde(rename = "proxyType")]
    pub kind: Kind,

    #[serde(rename = "proxyAddress")]
    pub address: Address<'a>,

    #[serde(rename = "proxyPort", serialize_with = "stringify")]
    pub port: u16,

    #[serde(rename = "proxyLogin", skip_serializing_if = "Option::is_none")]
    pub login: Option<Cow<'a, str>>,

    #[serde(rename = "proxyPassword", skip_serializing_if = "Option::is_none")]
    pub password: Option<Cow<'a, str>>,
}

/// The 2captcha API expects the proxy's port to be a string, but knowing it should
/// always be a number, we want [`Proxy::port`] to be of type [`u16`], which means
/// we need to manually serialize it as a string
#[allow(clippy::trivially_copy_pass_by_ref)]
fn stringify<S: Serializer>(value: &u16, serializer: S) -> Result<S::Ok, S::Error> {
    value.to_string().serialize(serializer)
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr};

    use serde_json::to_string;

    use super::{Address, Kind, Proxy};

    #[test]
    fn serialize_proxy() {
        let proxy = Proxy {
            kind: Kind::Http,
            address: Address::IpAddress(IpAddr::V4(Ipv4Addr::from([1, 2, 3, 4]))),
            port: 8080,
            login: Some("user23".into()),
            password: Some("p4$$w0rd".into()),
        };

        let expected = r#"{"proxyType":"http","proxyAddress":"1.2.3.4","proxyPort":"8080","proxyLogin":"user23","proxyPassword":"p4$$w0rd"}"#;

        assert_eq!(to_string(&proxy).unwrap(), expected);
    }
}
