//! Protocols for communicating with the server

#[cfg(feature = "http")]
pub(crate) mod http;
#[cfg(feature = "ws")]
pub(crate) mod ws;

use serde::Deserialize;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
#[derive(Debug)]
pub struct Http;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
#[derive(Debug)]
pub struct Https;

#[cfg(feature = "ws")]
#[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
#[derive(Debug)]
pub struct Ws;

#[cfg(feature = "ws")]
#[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
#[derive(Debug)]
pub struct Wss;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum Status {
    Ok,
    Err,
}
