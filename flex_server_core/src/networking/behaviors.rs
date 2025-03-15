use std::pin::Pin;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{connections::NetConnection, listeners::NetListener},
};

pub fn infinite_read<TConnection, TListener>() -> Box<
    dyn FnOnce(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>> + Send,
>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection> + Send + 'static,
{
    Box::new(|listener: TListener| {
        Box::pin(async move {
            loop {
                match listener.accept().await {
                    Ok(_conn) => log::info!("Got connection"),
                    Err(err) => log::error!("Error receiving connection: {err}"),
                }
            }

            #[allow(unreachable_code)]
            Ok(())
        })
    })
}
