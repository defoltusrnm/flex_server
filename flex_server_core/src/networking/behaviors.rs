use std::pin::Pin;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{connections::NetConnection, listeners::NetListener},
};

pub async fn infinite_read<
    TConnection: NetConnection,
    TListener: NetListener<TConnection> + std::marker::Send + 'static,
>(
    listener: TListener,
) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>> {
    Box::pin(async move {
        loop {
            match listener.accept().await {
                Ok(conn) => log::info!("got connection"),
                Err(err) => log::error!("\"{err}\" when receiving connection"),
            }
        }

        Ok(())
    })
}
