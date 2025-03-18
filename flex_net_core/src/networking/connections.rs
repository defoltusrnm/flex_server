use crate::error_handling::server_errors::ServerError;

use super::messages::NetMessage;

pub trait NetWriter
where
    Self: Send + Sized,
{
    fn write(self);
}

pub trait NetReader
where
    Self: Send + Sized,
{
    fn read(&mut self) -> impl Future<Output = Result<NetMessage, ServerError>>;
}

pub trait NetConnection: NetReader + NetWriter {}
