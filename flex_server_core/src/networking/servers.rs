use std::pin::Pin;

use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{
        address_src::EndpointAddressSrc, certificate_src::CertificateSrc, connections::NetConnection
    },
};

use super::listeners::NetListener;

pub trait NetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
{
    fn start<TEndpointAddrSrc>(
        src: TEndpointAddrSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>>>>,
        >,
    ) -> impl Future<Output = Result<(), ServerError>>
    where
        TEndpointAddrSrc: EndpointAddressSrc;
}


pub trait SecureNetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: NetListener<TConnection>,
{
    fn start<TEndpointAddrSrc, TCertificateSrc>(
        src: TEndpointAddrSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>>>>,
        >,
    ) -> impl Future<Output = Result<(), ServerError>>
    where
        TEndpointAddrSrc: EndpointAddressSrc,
        TCertificateSrc: CertificateSrc;
}