//! Clients for communicating with remote servers

#[cfg(feature = "protocol-http")]
#[cfg_attr(docsrs, doc(cfg(feature = "protocol-http")))]
pub use crate::protocol::http::Client as HttpClient;

#[cfg(feature = "protocol-ws")]
#[cfg_attr(docsrs, doc(cfg(feature = "protocol-ws")))]
pub use crate::protocol::ws::Client as WsClient;
