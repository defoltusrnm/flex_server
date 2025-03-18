mod app_logging;

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use flex_net_core::utils::env_host_source::EnvEndpointAddressSrc;
use flex_net_tcp::networking::{connections::NetTcpConnection, secure_connections::SecureNetTcpConnection};
use flex_server_core::{
    networking::{secure_server_behaviors, server_behaviors, servers::{NetServer, SecureNetServer}, session_behaviors},
    utils::{generic_server::GenericServer, secure_generic_server::SecureGenericServer},
};
use flex_server_tcp::{networking::{listeners::NetTcpListener, secure_listeners::SecureTcpNetListener}, utils::pkcs12_certificate_src::Pkcs12CertificateSrc};
use log::LevelFilter;

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    dotenv().unwrap();
    log::trace!(".env loaded");

    _secure_server().await;
}

async fn _insecure_server() {
    let server_handler = server_behaviors::infinite_read::<NetTcpConnection, NetTcpListener, _, _>(
        &session_behaviors::infinite_read::<NetTcpConnection>,
    );

    match GenericServer::start(
        EnvEndpointAddressSrc::new_with_port_fallback(4141),
        server_handler,
    )
    .await
    {
        Ok(()) => log::info!("server ended it's work"),
        Err(err) => log::error!("server ended it's work with: {err}"),
    }
}

async fn _secure_server() {
    let server_handler = secure_server_behaviors::infinite_read::<SecureNetTcpConnection, SecureTcpNetListener, _, _>(
        &session_behaviors::infinite_read::<SecureNetTcpConnection>,
    );

    match SecureGenericServer::start(
        EnvEndpointAddressSrc::new_with_port_fallback(4141),
        Pkcs12CertificateSrc::new_from_file_name_and_env("cert.pfx", "HOST"),
        server_handler,
    )
    .await
    {
        Ok(()) => log::info!("server ended it's work"),
        Err(err) => log::error!("server ended it's work with: {err}"),
    }
}
