use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Http,
    Socks4,
    Socks5,
}
