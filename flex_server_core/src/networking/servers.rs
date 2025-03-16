use std::sync::Arc;

use flex_net_core::{error_handling::server_errors::ServerError, networking::{address_src::EndpointAddressSrc, connections::NetConnection, listeners::NetListener}};

pub trait NetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
{
    async fn start<
        TEndpointAddrSrc: EndpointAddressSrc,
        F: Send + AsyncFn(TListener) -> Result<(), ServerError>       
    >(
        src: TEndpointAddrSrc,
        handler: Arc<F>,
    ) -> Result<(), ServerError>;
}