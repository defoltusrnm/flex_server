use flex_net_core::{error_handling::server_errors::ServerError, networking::connections::NetConnection};

pub async fn infinite_read<TConnection>(
    connection: &mut TConnection,
) -> Result<(), ServerError>
where
    TConnection: NetConnection,
{
    loop {
        match connection.read().await {
            Ok(_frame) => {
                log::info!("received message")
            },
            Err(err) => {
                return Err(err);
            }
        };
    }
}
