use flex_net_core::{error_handling::server_errors::ServerError, networking::{address_src::EndpointAddress, connections::NetConnection}};


pub trait NetListener<TConnection>
where
    TConnection: NetConnection,
    Self: Sized,
{
    fn bind(addr: EndpointAddress) -> impl Future<Output = Result<Self, ServerError>>;
    fn accept(&self) -> impl Future<Output = Result<TConnection, ServerError>>;
}
