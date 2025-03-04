#![allow(dead_code)]

use std::collections::HashMap;

use log::{debug, info};
use tokio::net::ToSocketAddrs;

use crate::protocol::{Handler, HttpProtocol, Protocol, ToHandler};
use crate::request::RequestMethod;
use crate::router::Router;
use crate::transport::{TcpTransport, Transport};
use crate::Result;

// TODO: some sort of trace/id for each connection for easier log reading
// TODO: builder for build YarsServer (picking transport/protocol)

// TODO: some sort of config file

// TODO: doc comment with example usage
pub struct YarsServer<T, P>
where
    T: Transport,
    P: Protocol,
{
    transport: T,
    protocol: P,
    router: Router<P>,
    handlers: HashMap<(String, RequestMethod), Box<Handler<P>>>,
    default_handler: Option<Box<Handler<P>>>,
}

impl YarsServer<TcpTransport, HttpProtocol> {
    /// Instantiates a HTTP server that accepts TCP connections
    pub fn default_server() -> Self {
        YarsServer {
            transport: TcpTransport::new(),
            protocol: HttpProtocol,
            router: Router::new(),
            handlers: HashMap::new(),
            default_handler: None,
        }
    }
}

impl<T, P> YarsServer<T, P>
where
    T: Transport,
    P: Protocol,
{
    pub fn new(transport: T, protocol: P) -> Self {
        Self {
            transport,
            protocol,
            router: Router::new(),
            handlers: HashMap::new(),
            default_handler: None,
        }
    }

    /// Adds a route with key `routing_key` that will call the given `handler`
    pub fn route(
        mut self,
        routing_key: impl Into<P::RoutingKey>,
        handler: impl ToHandler<P>,
    ) -> Self {
        self.router.add_route(routing_key.into(), handler);
        self
    }

    pub fn default_handler(mut self, handler: impl ToHandler<P>) -> Self {
        self.router.set_default_handler(handler);
        self
    }

    pub async fn listen<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        // TODO?: debug print type of transport and protocol
        debug!("{:#?}", self.router);

        let addr = self.transport.bind(addr).await?;

        info!("listening on {}", addr);

        // TODO?: tokio spawn or whatever for async
        loop {
            // Accept connection with transport layer
            let mut conn = self.transport.accept().await?;

            // TODO: break this down into smaller functions

            // Read request from connection with transport layer
            let raw_request = self.transport.read(&mut conn).await?;
            debug!("bytes read from connection {}: {}", addr, raw_request.len());

            // Parse request bytes using protocol layer
            let Some(request) = self.protocol.parse_request(&raw_request) else {
                debug!("Failed to parse request");
                continue;
            };

            // Extract routing key using protocol layer
            let routing_key = self.protocol.extract_routing_key(&request);
            info!("{}", routing_key);

            // TODO?: could impl middleware here

            // Get handler according to routing (according to protocol layer)
            let Some(handler) = self.router.get_request_handler(&routing_key) else {
                debug!("No handler found for: {}", routing_key);
                continue;
            };

            // Handle request by calling handler
            let response = handler(request);

            // Serialize response using protocol layer
            let response_bytes = self.protocol.serialize_response(&response);
            debug!(
                "writing bytes to connection {}: {}",
                addr,
                response_bytes.len()
            );

            // Write response bytes to connection with transport layer
            self.transport.write(&mut conn, &response_bytes).await?;
        }

        // TODO?
        // self.transport.close(conn).await?;
    }
}

macro_rules! http_method {
    ($method:ident, $request_method:ident) => {
        #[doc = concat!("Registers a `", stringify!($request_method), "` request handler that serves `path` by calling `handler`")]
        pub fn $method(self, path: &str, handler: impl ToHandler<HttpProtocol>) -> Self {
            self.route((path, RequestMethod::$request_method), handler)
        }
    };
}

/// HTTP specific methods
// TODO: some proc(?) macro(?) like #[get("/")] or #[post("/")]
// fn index() -> HttpResponse {
// will make the function into a struct that impls ToHandler
impl<T> YarsServer<T, HttpProtocol>
where
    T: Transport,
{
    http_method!(get, GET);
    http_method!(post, POST);
    http_method!(put, PUT);
    http_method!(delete, DELETE);
    http_method!(head, HEAD);
    http_method!(options, OPTIONS);
    http_method!(connect, CONNECT);
    http_method!(trace, TRACE);
    http_method!(patch, PATCH);
    // TODO?: files
}

// TODO: allow handlers that return Result<JSON, Error(?)>
// Really we don't have to do that here, we just need to
// impl Into<HttpResponse> for JSON
// But we DO need to handle funcs that return Result
// actually maybe not, kinda cba

// TODO?: serde, maybe make our own ToJson trait so user
// can use any json lib they want - that we support (with feature flags)
