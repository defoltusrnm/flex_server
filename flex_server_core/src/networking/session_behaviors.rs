use flex_net_core::{
    error_handling::server_errors::ServerError, networking::connections::NetConnection,
};
use std::pin::Pin;

pub fn infinite_read<'b,  TConnection>() -> Box<
    Fn(&'static mut TConnection) -> Pin<Box<dyn Future<Output = Result<(), ServerError>> + Send>>
        + Send + 'b,
>
where
    TConnection: NetConnection,
{
    Box::new(|connection| {
        Box::pin(async move {
            loop {
                connection.read().await;
            }

            Ok(())
        })
    })
}
