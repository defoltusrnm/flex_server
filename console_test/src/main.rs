mod app_logging;
mod networking;
mod server_errors;

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use flex_net_tcp::networking::connections::NetTcpConnection;
use flex_server_core::networking::{server_behaviors, servers::NetServer, session_behaviors};
use flex_server_tcp::networking::listeners::NetTcpListener;
use log::LevelFilter;
use networking::{
    address_src::EndpointAddressSrcs, 
    servers::GenericServer,
};

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    dotenv().unwrap();
    log::trace!(".env loaded");

    let server_handler = server_behaviors::infinite_read::<NetTcpConnection, NetTcpListener, _, _>(
        &session_behaviors::infinite_read::<NetTcpConnection>,
    );

    match GenericServer::start(EndpointAddressSrcs::env(), server_handler).await {
        Ok(()) => log::info!("server ended it's work"),
        Err(err) => log::error!("server ended it's work with: {err}"),
    }
}
