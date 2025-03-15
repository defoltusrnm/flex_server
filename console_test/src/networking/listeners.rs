use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{address_src::EndpointAddress, listeners::NetListener},
};
use tokio::net::TcpListener;

use crate::server_errors::errors::ServerErrors;

use super::connections::NetTcpConnection;

pub struct NetTcpListener {
    inner_listener: TcpListener,
}

impl NetListener<NetTcpConnection> for NetTcpListener {
    async fn bind(addr: EndpointAddress) -> Result<Self, ServerError> {
        match TcpListener::bind(format!("{0}:{1}", addr.host, addr.port)).await {
            Ok(listener) => Ok(NetTcpListener {
                inner_listener: listener,
            }),
            Err(err) => Err(ServerErrors::bind_error(err)),
        }
    }

    async fn accept(&self) -> Result<NetTcpConnection, ServerError> {
        match self.inner_listener.accept().await {
            Ok((socket, addr)) => Ok(NetTcpConnection::from_tcp_stream(socket, addr)),
            Err(err) => Err(ServerErrors::receive_error(err)),
        }
    }
}
