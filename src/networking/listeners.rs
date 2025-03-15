use tokio::net::TcpListener;

use crate::server_errors::errors::AppError;

use super::{
    address_src::EndpointAddress,
    connections::{NetConnection, NetTcpConnection},
};

pub trait NetListener<TConnection>
where
    TConnection: NetConnection,
    Self: Sized,
{
    fn bind(addr: EndpointAddress) -> impl std::future::Future<Output = Result<Self, AppError>> + Send;
    fn accept(&self) -> impl std::future::Future<Output = Result<TConnection, AppError>> + Send;
}

pub struct NetTcpListener {
    inner_listener: TcpListener,
}

impl NetListener<NetTcpConnection> for NetTcpListener {
    async fn bind(addr: EndpointAddress) -> Result<Self, AppError> {
        match TcpListener::bind(format!("{0}:{1}", addr.host, addr.port)).await {
            Ok(listener) => Ok(NetTcpListener {
                inner_listener: listener,
            }),
            Err(err) => Err(AppError::BindError(err)),
        }
    }

    async fn accept(&self) -> Result<NetTcpConnection, AppError> {
        match self.inner_listener.accept().await {
            Ok((socket, addr)) => Ok(NetTcpConnection::from_tcp_stream(socket, addr)),
            Err(err) => Err(AppError::ReceiveError(err)),
        }
    }
}
