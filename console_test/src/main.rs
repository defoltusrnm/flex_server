mod app_logging;
mod networking;
mod server_errors;

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use flex_server_core::networking::{server_behaviors, servers::NetServer, session_behaviors};
use log::{LevelFilter, error, info, trace};
use networking::{address_src::EndpointAddressSrcs, connections::NetTcpConnection, listeners::NetTcpListener, servers::ContinuesServer};

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    dotenv().unwrap();
    trace!(".env loaded");

    match ContinuesServer::start(
        EndpointAddressSrcs::env() 
    ).await
    {
        Ok(()) => info!("server closed"),
        Err(app_err) => error!("server got critical err: {app_err}"),
    }
}
