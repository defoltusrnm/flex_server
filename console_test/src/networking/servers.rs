use std::pin::Pin;

use flex_net_core::{
    async_utils::async_and_then::AsyncAndThen,
    error_handling::server_errors::ServerError,
    networking::{
        address_src::EndpointAddressSrc, connections::NetConnection, listeners::NetListener,
    },
};

pub trait NetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
{
    async fn start<
        TEndpointAddrSrc: EndpointAddressSrc,
        F: 'static
            + FnOnce(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>>
            + Send,
    >(
        src: TEndpointAddrSrc,
        handler: F,
    ) -> Result<(), ServerError>;
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
            + FnOnce(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>>
            + Send,
    >(
        src: TEndpointAddrSrc,
        handler: F,
    ) -> Result<(), ServerError> {
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
