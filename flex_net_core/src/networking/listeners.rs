use crate::error_handling::server_errors::ServerError;

use super::{address_src::EndpointAddress, connections::NetConnection};

pub trait NetListener<TConnection>
where
    TConnection: NetConnection,
    Self: Sized,
{
    async fn bind(addr: EndpointAddress) -> Result<Self, ServerError>;
    async fn accept(&self) -> Result<TConnection, ServerError>;
}
