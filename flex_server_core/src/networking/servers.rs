use std::pin::Pin;

use flex_net_core::{error_handling::server_errors::ServerError, networking::{address_src::EndpointAddressSrc, connections::NetConnection, listeners::NetListener}};

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