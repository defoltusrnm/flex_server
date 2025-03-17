use std::sync::Arc;

use flex_net_core::{
    async_utils::async_and_then::AsyncAndThen,
    error_handling::server_errors::ServerError,
    networking::{
        address_src::EndpointAddressSrc, connections::NetConnection, listeners::NetListener,
    },
};
use flex_server_core::networking::servers::NetServer;

pub struct GenericServer;

impl<TConnection, TListener> NetServer<TConnection, TListener> for GenericServer
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection> + Send,
{
    async fn start<TEndpointAddrSrc, ListenerFunc, ConnFunc>(
        src: TEndpointAddrSrc,
        listener_handler: ListenerFunc,
        connection_handler: ConnFunc,
    ) -> Result<(), ServerError>
    where
        TEndpointAddrSrc: EndpointAddressSrc,
        ListenerFunc: AsyncFn(TListener, ConnFunc) -> Result<(), ServerError>,
        ConnFunc: AsyncFn(&mut TConnection) -> Result<(), ServerError>,
    {
        let x = src
            .get()
            .inspect(|addr| log::info!("server will try to use {0}:{1}", addr.host, addr.port))
            .and_then_async(|addr| TListener::bind(addr))
            .await
            .inspect(|_| log::info!("server ready to receive new connections"))
            .and_then_async(async move |listener| listener_handler(listener, connection_handler).await)
            .await;

        x
    }
}
