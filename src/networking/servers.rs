use std::pin::Pin;

use super::{address_src::EndpointAddressSrc, connections::NetConnection, listeners::NetListener};
use crate::{async_utils::async_and_then::AsyncAndThen, server_errors::errors::AppError};

pub trait NetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
{
    async fn start<
        TEndpointAddrSrc: EndpointAddressSrc,
        F: 'static
            + FnOnce(TListener) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>
            + Send,
    >(
        src: TEndpointAddrSrc,
        handler: F,
    ) -> Result<(), AppError>;
}

pub struct ContinuesServer;

impl<TConnection, TListener> NetServer<TConnection, TListener> for ContinuesServer
where
    TListener: Send,
    TConnection: NetConnection,
    TListener: NetListener<TConnection> + 'static,
{
    async fn start<
        TEndpointAddrSrc: EndpointAddressSrc,
        F: 'static
            + FnOnce(TListener) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>
            + Send,
    >(
        src: TEndpointAddrSrc,
        handler: F,
    ) -> Result<(), AppError> {
        let server_result = src
            .get()
            .and_then_async(|addr| Box::pin(async move { TListener::bind(addr).await }))
            .await
            .inspect(|_| log::info!("server ready to accept new connections"))
            .and_then_async(handler)
            .await;

        server_result
    }
}
