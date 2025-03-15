mod app_logging;
mod networking;
mod server_errors;

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use flex_net_core::networking::listeners::NetListener;
use flex_server_core::networking::servers::NetServer;
use log::{LevelFilter, error, info, trace};
use networking::{address_src::EndpointAddressSrcs, listeners::NetTcpListener, servers::ContinuesServer};

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    dotenv().unwrap();
    trace!(".env loaded");

    match ContinuesServer::start(EndpointAddressSrcs::env(), |listener: NetTcpListener| {
        Box::pin(async move {
            loop {
                match listener.accept().await {
                    Ok(conn) => info!("got connection"),
                    Err(err) => error!("\"{err}\" when receiving connection"),
                }
            }

            Ok(())
        })
    })
    .await
    {
        Ok(()) => info!("server closed"),
        Err(app_err) => error!("server got critical err: {app_err}"),
    }
}
