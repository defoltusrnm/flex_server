use std::pin::Pin;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{connections::NetConnection, listeners::NetListener},
};

pub fn infinite_read<'b : 'static, TConnection, TListener>(
    session_reader: Box<
        dyn Fn(
                &'static mut TConnection,
            ) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>>
            + Send + 'b,
    >,
) -> Box<
    dyn FnOnce(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>> + Send,
>
where
    TConnection: NetConnection + 'static,
    TListener: NetListener<TConnection> + Send + 'static,
{
    Box::new(|listener: TListener| {
        Box::pin(async move {
            loop {
                match listener.accept().await {
                    Ok(mut conn) => {
                        log::info!("Got connection");
                        let session_res = session_reader(&mut conn).await;

                        match session_res {
                            Ok(()) => log::info!("connection died"),
                            Err(err) => log::error!("connection died: {err}")                            
                        };
                    }
                    Err(err) => log::error!("Error receiving connection: {err}"),
                }
            }

            #[allow(unreachable_code)]
            Ok(())
        })
    })
}
