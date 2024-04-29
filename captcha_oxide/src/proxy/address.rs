use std::{borrow::Cow, net::IpAddr};

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Address<'a> {
    IpAddress(IpAddr),
    HostName(Cow<'a, str>),
}
