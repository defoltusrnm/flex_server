use std::pin::Pin;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{
        connections::NetConnection,
        listeners::{self, NetListener},
    },
};

pub fn infinite_read<'a, TConnection, TListener, ConnFunc>(
    connection_handler: &'a ConnFunc,
) -> Box<dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + 'a>> + 'a>
where
    TConnection: 'a + NetConnection,
    TListener: 'a + NetListener<TConnection>,
    ConnFunc: 'a + AsyncFn(&mut TConnection) -> Result<(), ServerError>,
{
    Box::new(move |l| infinite_read_pin(l, connection_handler))
}

fn infinite_read_pin<'a, TConnection, TListener, ConnFunc>(
    listener: TListener,
    connection_handler: ConnFunc,
) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + 'a>>
where
    TConnection: 'a + NetConnection,
    TListener: 'a + NetListener<TConnection>,
    ConnFunc: 'a + AsyncFn(&mut TConnection) -> Result<(), ServerError>,
{
    Box::pin(infinite_read_impl(listener, connection_handler))
}

pub async fn infinite_read_impl<TConnection, TListener, ConnFunc>(
    listener: TListener,
    connection_handler: ConnFunc,
) -> Result<(), ServerError>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
    ConnFunc: AsyncFn(&mut TConnection) -> Result<(), ServerError>,
{
    loop {
        log::info!("waiting for new connections");
        match listener.accept().await {
            Ok(mut connection) => {
                log::info!("got connection");

                match connection_handler(&mut connection).await {
                    Ok(()) => {
                        log::info!("connection handled")
                    }
                    Err(err) => {
                        log::error!("connection ended with error: {err}")
                    }
                };
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}
