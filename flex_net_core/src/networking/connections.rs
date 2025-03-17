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
    async fn read(&mut self) -> Result<NetMessage, ServerError>;
}

pub trait NetConnection: NetReader + NetWriter {}
