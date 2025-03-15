use std::env;

use flex_net_core::{error_handling::server_errors::ServerError, networking::address_src::{EndpointAddress, EndpointAddressSrc}};

use crate::server_errors::errors::ServerErrors;

pub struct EndpointAddressSrcs;

impl EndpointAddressSrcs {
    pub fn env() -> EnvEndpointAddressSrc {
        EnvEndpointAddressSrc {}
    }
}

pub struct EnvEndpointAddressSrc;

impl EndpointAddressSrc for EnvEndpointAddressSrc {
    fn get(self) -> Result<EndpointAddress, ServerError> {
        env::var("HOST")
            .map_err(|err| ServerErrors::host_is_not_provided(err))
            .map(|host| {
                let port = env::var("PORT")
                    .map_err(|x| ServerErrors::port_env_read_error(x))
                    .and_then(|x| {
                        x.parse::<i32>()
                            .map_err(|err| ServerErrors::port_parse_error(err))
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
