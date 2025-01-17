use super::Client;
use super::HttpRoute;
use crate::api::param::from_value;
use crate::api::param::DbResponse;
use crate::api::param::Param;
use crate::api::param::ServerAddrs;
#[cfg(any(feature = "native-tls", feature = "rustls"))]
use crate::api::param::Tls;
use crate::api::Connection;
use crate::api::Method;
use crate::api::QueryResponse;
use crate::api::Result;
use crate::api::Route;
use crate::api::Router;
use crate::api::Surreal;
use flume::Receiver;
use futures::StreamExt;
use indexmap::IndexMap;
use once_cell::sync::OnceCell;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::ACCEPT;
use reqwest::ClientBuilder;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
#[cfg(feature = "protocol-ws")]
use std::sync::atomic::AtomicI64;
use std::sync::Arc;
use url::Url;

impl Connection for Client {
	type Request = (Method, Param);
	type Response = Result<DbResponse>;

	fn new(method: Method) -> Self {
		Self {
			method,
		}
	}

	fn connect(
		address: ServerAddrs,
		capacity: usize,
	) -> Pin<Box<dyn Future<Output = Result<Surreal<Self>>> + Send + Sync + 'static>> {
		Box::pin(async move {
			let mut headers = HeaderMap::new();
			headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

			#[allow(unused_mut)]
			let mut builder = ClientBuilder::new().default_headers(headers);

			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			if let Some(tls) = address.tls_config {
				builder = match tls {
					#[cfg(feature = "native-tls")]
					Tls::Native(config) => builder.use_preconfigured_tls(config),
					#[cfg(feature = "rustls")]
					Tls::Rust(config) => builder.use_preconfigured_tls(config),
				};
			}

			let client = builder.build()?;

			let base_url = address.endpoint;

			super::health(client.get(base_url.join(Method::Health.as_str())?)).await?;

			let (route_tx, route_rx) = match capacity {
				0 => flume::unbounded(),
				capacity => flume::bounded(capacity),
			};

			router(base_url, client, route_rx);

			Ok(Surreal {
				router: OnceCell::with_value(Arc::new(Router {
					conn: PhantomData,
					sender: route_tx,
					#[cfg(feature = "protocol-ws")]
					last_id: AtomicI64::new(0),
				})),
			})
		})
	}

	fn send<'r>(
		&'r mut self,
		router: &'r Router<Self>,
		param: Param,
	) -> Pin<Box<dyn Future<Output = Result<Receiver<Self::Response>>> + Send + Sync + 'r>> {
		Box::pin(async move {
			let (sender, receiver) = flume::bounded(1);
			let route = Route {
				request: (self.method, param),
				response: sender,
			};
			router.sender.send_async(Some(route)).await?;
			Ok(receiver)
		})
	}

	fn recv<R>(
		&mut self,
		rx: Receiver<Self::Response>,
	) -> Pin<Box<dyn Future<Output = Result<R>> + Send + Sync + '_>>
	where
		R: DeserializeOwned,
	{
		Box::pin(async move {
			let response = rx.into_recv_async().await?;
			tracing::trace!("Response {response:?}");
			match response? {
				DbResponse::Other(value) => from_value(&value),
				DbResponse::Query(..) => unreachable!(),
			}
		})
	}

	fn recv_query(
		&mut self,
		rx: Receiver<Self::Response>,
	) -> Pin<Box<dyn Future<Output = Result<QueryResponse>> + Send + Sync + '_>> {
		Box::pin(async move {
			let response = rx.into_recv_async().await?;
			tracing::trace!("Response {response:?}");
			match response? {
				DbResponse::Query(results) => Ok(results),
				DbResponse::Other(..) => unreachable!(),
			}
		})
	}
}

fn router(base_url: Url, client: reqwest::Client, route_rx: Receiver<Option<HttpRoute>>) {
	tokio::spawn(async move {
		let mut headers = HeaderMap::new();
		let mut vars = IndexMap::new();
		let mut auth = None;
		let mut stream = route_rx.into_stream();

		while let Some(Some(route)) = stream.next().await {
			match super::router(
				route.request,
				&base_url,
				&client,
				&mut headers,
				&mut vars,
				&mut auth,
			)
			.await
			{
				Ok(value) => {
					let _ = route.response.into_send_async(Ok(value)).await;
				}
				Err(error) => {
					let _ = route.response.into_send_async(Err(error)).await;
				}
			}
		}
	});
}
