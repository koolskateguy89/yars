use super::YarsServer;
use crate::{
    protocol::{HttpProtocol, Protocol},
    transport::{TcpTransport, Transport},
};

pub struct NoTransport;

pub struct HasTransport<T>(T);

pub struct NoProtocol;

pub struct HasProtocol<P>(P);

pub struct YarsServerBuilder<T, P> {
    transport: T,
    protocol: P,
}

impl YarsServerBuilder<NoTransport, NoProtocol> {
    pub(crate) fn new() -> Self {
        Self {
            transport: NoTransport,
            protocol: NoProtocol,
        }
    }

    pub fn transport<T>(self, transport: T) -> YarsServerBuilder<HasTransport<T>, NoProtocol>
    where
        T: Transport,
    {
        YarsServerBuilder {
            transport: HasTransport(transport),
            protocol: NoProtocol,
        }
    }

    pub fn tcp(self) -> YarsServerBuilder<HasTransport<TcpTransport>, NoProtocol> {
        self.transport(TcpTransport::new())
    }
}

impl<T> YarsServerBuilder<HasTransport<T>, NoProtocol> {
    pub fn protocol<P>(self, protocol: P) -> YarsServerBuilder<HasTransport<T>, HasProtocol<P>>
    where
        P: Protocol,
    {
        YarsServerBuilder {
            transport: self.transport,
            protocol: HasProtocol(protocol),
        }
    }

    pub fn http(self) -> YarsServerBuilder<HasTransport<T>, HasProtocol<HttpProtocol>> {
        self.protocol(HttpProtocol)
    }
}

impl<T, P> YarsServerBuilder<HasTransport<T>, HasProtocol<P>>
where
    T: Transport,
    P: Protocol,
{
    pub fn build(self) -> YarsServer<T, P> {
        YarsServer::new(self.transport.0, self.protocol.0)
    }
}
