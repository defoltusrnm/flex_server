use std::{pin::Pin, sync::Arc};

use flex_net_core::{
    async_utils::async_and_then::AsyncAndThen,
    error_handling::server_errors::ServerError,
    networking::{
        address_src::EndpointAddressSrc, connections::NetConnection, listeners::NetListener,
    },
};
use flex_server_core::networking::servers::NetServer;

pub struct ContinuesServer;

impl<TConnection, TListener> NetServer<TConnection, TListener> for ContinuesServer
where
    TListener: Send,
    TConnection: NetConnection,
    TListener: NetListener<TConnection> + 'static,
{
    async fn start<
        TEndpointAddrSrc: EndpointAddressSrc,
        F: Send + AsyncFn(TListener) -> Result<(), ServerError>
    >(
        src: TEndpointAddrSrc,
        handler: Arc<F>,
    ) -> Result<(), ServerError> {
        let server_result = src
            .get()
            .and_then_async(|addr| Box::pin(async move { TListener::bind(addr).await }))
            .await
            .inspect(|_| log::info!("server ready to accept new connections"))
            .map(|_| {()});

        server_result
    }
}
