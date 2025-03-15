use crate::error_handling::server_errors::ServerError;

use super::{address_src::EndpointAddress, connections::NetConnection};

pub trait NetListener<TConnection>
where
    TConnection: NetConnection,
    Self: Sized,
{
    fn bind(addr: EndpointAddress) -> impl std::future::Future<Output = Result<Self, ServerError>> + Send;
    fn accept(&self) -> impl std::future::Future<Output = Result<TConnection, ServerError>> + Send;
}
