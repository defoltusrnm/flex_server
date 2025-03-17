mod app_logging;
mod networking;
mod server_errors;

use std::{os::windows::io::AsRawHandle, time::Duration};

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use flex_server_core::networking::{server_behaviors, servers::NetServer, session_behaviors};
use log::LevelFilter;
use networking::{
    address_src::EndpointAddressSrcs, connections::NetTcpConnection, listeners::NetTcpListener,
    servers::GenericServer,
};

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    dotenv().unwrap();
    log::trace!(".env loaded");

    match GenericServer::start(
        EndpointAddressSrcs::env(),
        server_behaviors::infinite_read::<NetTcpConnection, NetTcpListener, _>,
        session_behaviors::infinite_read::<NetTcpConnection>
    )
    .await {
        Ok(()) => log::info!("server ended it's work"),
        Err(err) => log::error!("server ended it's work with: {err}")
    }
}
