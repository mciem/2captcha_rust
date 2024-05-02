use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Http,
    Socks4,
    Socks5,
}
