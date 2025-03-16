use std::net::SocketAddr;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{
        connections::{NetConnection, NetReader, NetWriter},
        messages::NetMessage,
    },
};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::server_errors::errors::ServerErrors;

pub struct NetTcpConnection {
    inner_socket: TcpStream,
    call_addr: SocketAddr,
}

impl NetTcpConnection {
    pub fn from_tcp_stream(stream: TcpStream, call_addr: SocketAddr) -> NetTcpConnection {
        NetTcpConnection {
            inner_socket: stream,
            call_addr,
        }
    }
}

impl NetConnection for NetTcpConnection {}

impl NetReader for NetTcpConnection {
    async fn read(&mut self) -> Result<NetMessage, ServerError> {
        let mut buff = vec![0u8; 512];

        match self.inner_socket.read(&mut buff).await {
            Ok(len) => {
                buff.truncate(len);
                Ok(NetMessage::new(buff))
            }
            Err(err) => Err(ServerErrors::buffer_read_error(err)),
        }
    }
}

impl NetWriter for NetTcpConnection {
    fn write(self) {
        todo!()
    }
}
