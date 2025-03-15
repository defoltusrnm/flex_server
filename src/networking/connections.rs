use std::net::SocketAddr;

use tokio::net::TcpStream;

pub trait NetWriter
where
    Self: Send + Sized,
{
    fn write();
}

pub trait NetReader
where
    Self: Send + Sized,
{
    fn read();
}

pub trait NetConnection: NetReader + NetWriter {}

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
