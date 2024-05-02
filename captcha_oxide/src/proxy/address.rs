use std::net::IpAddr;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Address<'a> {
    IpAddress(IpAddr),
    HostName(&'a str),
}
