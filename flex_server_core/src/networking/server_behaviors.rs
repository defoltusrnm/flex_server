use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{connections::NetConnection, listeners::NetListener},
};

pub async fn infinite_read<TConnection, TListener, ConnFunc>(
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
