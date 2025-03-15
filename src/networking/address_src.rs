use crate::server_errors::errors::AppError;
use std::env;

pub struct EndpointAddressSrcs;

impl EndpointAddressSrcs {
    pub fn env() -> EnvEndpointAddressSrc {
        EnvEndpointAddressSrc {}
    }
}

pub struct EndpointAddress {
    pub host: String,
    pub port: i32,
}

impl EndpointAddress {
    fn from_ip_and_port(host: String, port: i32) -> EndpointAddress {
        EndpointAddress { host, port }
    }
}

pub trait EndpointAddressSrc {
    fn get(self) -> Result<EndpointAddress, AppError>;
}

pub struct EnvEndpointAddressSrc;

impl EndpointAddressSrc for EnvEndpointAddressSrc {
    fn get(self) -> Result<EndpointAddress, AppError> {
        env::var("HOST")
            .map_err(|err| AppError::HostIsNotProvided(err))
            .map(|host| {
                let port = env::var("PORT")
                    .map_err(|x| AppError::PortEnvReadError(x))
                    .and_then(|x| {
                        x.parse::<i32>()
                            .map_err(|err| AppError::PortParseError(err))
                    })
                    .unwrap_or_else(|err| {
                        log::error!(
                            "error occurred when reading port \"{err}\", will take default"
                        );
                        4141
                    });

                EndpointAddress::from_ip_and_port(host, port)
            })
    }
}
