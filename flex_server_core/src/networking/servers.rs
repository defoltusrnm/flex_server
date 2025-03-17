use std::sync::Arc;

use flex_net_core::{
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
    async fn start<TEndpointAddrSrc, ListenerFunc, ConnFunc>(
        src: TEndpointAddrSrc,
        listener_handler: ListenerFunc,
        connection_handler: ConnFunc,
    ) -> Result<(), ServerError>
    where
        TEndpointAddrSrc: EndpointAddressSrc,
        ListenerFunc: AsyncFn(TListener, ConnFunc) -> Result<(), ServerError>,
        ConnFunc: AsyncFn(&mut TConnection) -> Result<(), ServerError>;
}
