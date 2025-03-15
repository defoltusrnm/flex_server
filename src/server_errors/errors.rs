use derive_more::Display;

#[derive(Display)]
pub enum AppError {
    #[display("host addr could not be provided: {_0}")]
    HostIsNotProvided(std::env::VarError),
    #[display("could not read port from env: {_0}")]
    PortEnvReadError(std::env::VarError),
    #[display("could not parse port from env: {_0}")]
    PortParseError(std::num::ParseIntError),
    #[display("cannot start server because: {_0}")]
    BindError(std::io::Error),
    #[display("error when server tried to accept connection: {_0}")]
    ReceiveError(std::io::Error)
}