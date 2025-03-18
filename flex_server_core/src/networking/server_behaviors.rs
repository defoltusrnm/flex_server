use std::pin::Pin;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::connections::NetConnection,
};
use tokio::task;

use super::listeners::NetListener;

pub fn infinite_read<'a, TConnection, TListener, ConnFunc, ConnFut>(
    connection_handler: &'a ConnFunc,
) -> Box<dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + 'a>> + 'a>
where
    TConnection: 'a + NetConnection,
    TListener: 'a + NetListener<TConnection>,
    ConnFunc: Fn(TConnection) -> ConnFut,
    ConnFut: 'static + Send + Future<Output = Result<(), ServerError>>,
{
    Box::new(move |l| infinite_read_pin(l, connection_handler))
}

fn infinite_read_pin<'a, TConnection, TListener, ConnFunc, ConnFut>(
    listener: TListener,
    connection_handler: ConnFunc,
) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + 'a>>
where
    TConnection: 'a + NetConnection,
    TListener: 'a + NetListener<TConnection>,
    ConnFunc: 'a + Fn(TConnection) -> ConnFut,
    ConnFut: 'static + Send + Future<Output = Result<(), ServerError>>,
{
    Box::pin(infinite_read_impl(listener, connection_handler))
}

pub async fn infinite_read_impl<TConnection, TListener, ConnFunc, ConnFut>(
    listener: TListener,
    connection_handler: ConnFunc,
) -> Result<(), ServerError>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
    ConnFunc: Fn(TConnection) -> ConnFut,
    ConnFut: 'static + Send + Future<Output = Result<(), ServerError>>,
{
    let mut set = task::JoinSet::new();
    loop {
        log::info!("waiting for new connections");
        match listener.accept().await {
            Ok(connection) => {
                log::info!("got connection");

                set.spawn(connection_handler(connection));
            }
            Err(err) => {

                set.join_all().await.iter().for_each(|res| match res {
                    Ok(()) => log::info!("connection handled"),
                    Err(err) => log::error!("connection handled with: {err}"),
                });

                return Err(err);
            }
        };
    }
}
