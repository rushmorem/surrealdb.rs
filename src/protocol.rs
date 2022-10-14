#[cfg(feature = "http")]
pub(crate) mod http;
#[cfg(feature = "ws")]
pub(crate) mod ws;

#[cfg(feature = "http")]
pub use http::Http;
#[cfg(all(feature = "http", feature = "tls"))]
pub use http::Https;
#[cfg(feature = "ws")]
pub use ws::Ws;
#[cfg(all(feature = "ws", feature = "tls"))]
pub use ws::Wss;
