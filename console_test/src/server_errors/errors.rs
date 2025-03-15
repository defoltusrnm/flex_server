use flex_net_core::error_handling::server_errors::ServerError;

pub struct ServerErrors;

impl ServerErrors {
    pub fn host_is_not_provided(err: std::env::VarError) -> ServerError {
        ServerError::new(format!("host addr could not be provided: {err}"))
    }

    pub fn port_env_read_error(err: std::env::VarError) -> ServerError {
        ServerError::new(format!("could not read port from env: {err}"))
    }

    pub fn port_parse_error(err: std::num::ParseIntError) -> ServerError {
        ServerError::new(format!("could not parse port from env: {err}"))
    }

    pub fn bind_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!("cannot start server because: {err}"))
    }

    pub fn receive_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!(
            "error when server tried to accept connection: {err}"
        ))
    }
}
