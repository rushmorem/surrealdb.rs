#[cfg(feature = "http")]
mod http;
#[cfg(feature = "ws")]
mod ws;

use crate::Connection;
use crate::Result;
use url::Url;

#[cfg(any(feature = "native-tls", feature = "rustls"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "native-tls", feature = "rustls"))))]
#[derive(Debug)]
pub enum Tls {
    #[cfg(feature = "native-tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
    Native(native_tls::TlsConnector),
    #[cfg(feature = "rustls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
    Rust(rustls::ClientConfig),
}

#[derive(Debug)]
pub struct ServerAddrs {
    pub(crate) endpoint: Url,
    #[cfg(any(feature = "native-tls", feature = "rustls"))]
    pub(crate) tls_config: Option<Tls>,
}

pub trait ToServerAddrs<T> {
    type Client: Connection;

    fn to_server_addrs(self) -> Result<ServerAddrs>;
}
