use std::net::SocketAddr;

use flex_net_core::networking::connections::{NetConnection, NetReader, NetWriter};
use tokio::net::TcpStream;

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
    fn read() {
        todo!()
    }
}

impl NetWriter for NetTcpConnection {
    fn write() {
        todo!()
    }
}
