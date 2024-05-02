use std::{borrow::Cow, fmt::Debug};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use url::Url;

use crate::{
    captcha::{captcha, Empty},
    proxy::Proxy,
};

#[derive(Debug)]
#[captcha(
    crate = "crate",
    timeout = 20,
    solution = "GeeTestV4Solution<'a>",
    proxy(
        no_serde,
        with_proxy = "GeeTestTask",
        without_proxy = "GeeTestTaskProxyless",
    )
)]
pub struct GeeTestV4<'a, T = Empty>
where
    T: Serialize + Debug + Send + Sync,
{
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    website_url: Url,

    /// Custom GeeTest API domain, for example: `api-na.geetest.com`.
    /// Can be defined inside `initGeetest` call. Also you can check
    /// the domain used to load the scripts, the default domain is
    /// `api.geetest.com`.
    geetest_api_server_subdomain: Option<&'a str>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    user_agent: Option<&'a str>,

    /// Captcha parameters passed to `initGeetest`
    init_parameters: InitParameters<'a, T>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitParameters<'a, T> {
    captcha_id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    data: Option<T>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GeeTestV4Solution<'a> {
    pub captcha_id: Cow<'a, str>,
    pub lot_number: Cow<'a, str>,
    pub pass_token: Cow<'a, str>,
    pub gen_time: Cow<'a, str>,
    pub captcha_output: Cow<'a, str>,
}

impl<'a, T> GeeTestV4<'a, T>
where
    T: Serialize + Debug + Send + Sync,
{
    fn field_count(&self) -> usize {
        /// Accounts for [`GeeTestV4::website_url`], [`GeeTestV4::init_parameters`]
        /// and the two extra required fields that are not directly given by the user,
        /// which are "version" and "type"
        const REQUIRED_FIELD_COUNT: usize = 4;

        let optional_field_count = self.geetest_api_server_subdomain.map_or(0, |_| 1)
            + self.user_agent.map_or(0, |_| 1)
            + match self.proxy {
                ProxyTask::WithProxy(ref p) => {
                    /// Accounts for [`Proxy::port`], [`Proxy::address`] and [`Proxy::kind`]
                    const PROXY_REQUIRED_FIELD_COUNT: usize = 3;

                    let proxy_optional_field_count =
                        p.login.map_or(0, |_| 1) + p.password.map_or(0, |_| 1);

                    PROXY_REQUIRED_FIELD_COUNT + proxy_optional_field_count
                }
                ProxyTask::ProxyLess => 0,
            };

        REQUIRED_FIELD_COUNT + optional_field_count
    }
}

// Geetest v4 has a few quirks that require custom serialization, most notably,
// the version field, which must always be equal to 4
impl<'a, T> Serialize for GeeTestV4<'a, T>
where
    T: Serialize + Debug + Send + Sync,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GeeTestV4", self.field_count())?;

        match self.proxy {
            ProxyTask::WithProxy(_) => state.serialize_field("type", "GeeTestTask")?,
            ProxyTask::ProxyLess => {
                state.serialize_field("type", "GeeTestTaskProxyless")?;
            }
        }

        state.serialize_field("websiteURL", &self.website_url)?;

        if let Some(subdomain) = self.geetest_api_server_subdomain {
            state.serialize_field("geetestApiServerSubdomain", subdomain)?;
        } else {
            state.skip_field("geetestApiServerSubdomain")?;
        }

        if let Some(user_agent) = self.user_agent {
            state.serialize_field("userAgent", user_agent)?;
        } else {
            state.skip_field("userAgent")?;
        }

        state.serialize_field("version", &4_u8)?;
        state.serialize_field("initParameters", &self.init_parameters)?;
        if let ProxyTask::WithProxy(Proxy {
            port,
            kind,
            ref address,
            login,
            password,
        }) = self.proxy
        {
            state.serialize_field("proxyType", &kind)?;
            state.serialize_field("proxyAddress", address)?;
            state.serialize_field("proxyPort", &port.to_string())?;

            if let Some(login) = login {
                state.serialize_field("proxyLogin", login)?;
            } else {
                state.skip_field("proxyLogin")?;
            }

            if let Some(password) = password {
                state.serialize_field("proxyPassword", password)?;
            } else {
                state.skip_field("proxyPassword")?;
            }
        }

        state.end()
    }
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr};

    use url::Url;

    use super::{GeeTestV4, InitParameters};
    use crate::{
        proxy::{Address, Kind, Proxy},
        Captcha, Error,
    };

    #[test]
    fn serialization() -> Result<(), Error> {
        let proxyless = <GeeTestV4>::builder()
            .website_url(Url::parse("https://2captcha.com/demo/geetest-v4")?)
            .init_parameters(InitParameters {
                captcha_id: "e392e1d7fd421dc63325744d5a2b9c73",
                data: None,
            })
            .build();

        let with_proxy = <GeeTestV4>::builder()
            .website_url(Url::parse("https://2captcha.com/demo/geetest-v4")?)
            .init_parameters(InitParameters {
                captcha_id: "e392e1d7fd421dc63325744d5a2b9c73",
                data: None,
            })
            .proxy(Proxy {
                kind: Kind::Http,
                address: Address::IpAddress(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))),
                port: 8080,
                login: Some("user23"),
                password: Some("p4$w0rd"),
            })
            .build();

        assert_eq!(
            serde_json::to_string(&proxyless).unwrap(),
            r#"{"type":"GeeTestTaskProxyless","websiteURL":"https://2captcha.com/demo/geetest-v4","version":4,"initParameters":{"captcha_id":"e392e1d7fd421dc63325744d5a2b9c73"}}"#
        );

        assert_eq!(
            serde_json::to_string(&with_proxy).unwrap(),
            r#"{"type":"GeeTestTask","websiteURL":"https://2captcha.com/demo/geetest-v4","version":4,"initParameters":{"captcha_id":"e392e1d7fd421dc63325744d5a2b9c73"},"proxyType":"http","proxyAddress":"1.2.3.4","proxyPort":"8080","proxyLogin":"user23","proxyPassword":"p4$w0rd"}"#
        );

        Ok(())
    }
}
