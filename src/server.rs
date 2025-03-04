#![allow(dead_code)]

use log::{debug, info};
use std::collections::HashMap;

use tokio::net::ToSocketAddrs;

use crate::protocol::{Handler, HttpProtocol, Protocol, ToHandler};
use crate::request::RequestMethod;
use crate::router::Router;
use crate::transport::{TcpTransport, Transport};
use crate::Result;

// todo: some sort of trace/id for each connection for easier log reading

// TODO: some sort of config file

// TODO: directly import handler from protocol once done with generic impl.
// TODO?: some sort of builder for picking transport/protocol
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

    /// Adds a route with the given `path` and `method` that will call the given `handler`
    pub fn route<H>(mut self, routing_key: P::RoutingKey, handler: H) -> Self
    where
        H: ToHandler<P>,
    {
        self.router.add_route(routing_key, handler);
        self
    }

    pub fn default_handler<H>(mut self, handler: H) -> Self
    where
        H: ToHandler<P>,
    {
        self.router.set_default_handler(handler);
        self
    }

    pub async fn listen<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        // TODO?: debug print type of transport and protocol
        debug!("{:#?}", self.router);

        let addr = self.transport.bind(addr).await?;

        info!("listening on {}", addr);

        // TODO?: tokio spawn or whatever
        loop {
            // Accept connection with transport layer
            let mut conn = self.transport.accept().await?;

            // Read request from connection with transport layer
            let raw_request = self.transport.read(&mut conn).await?;
            debug!("bytes read from connection {}: {}", addr, raw_request.len());

            // Parse request bytes using protocol layer
            let Some(request) = self.protocol.parse_request(&raw_request) else {
                debug!("Failed to parse request; continuing to next connection");
                continue;
            };

            // Extract routing key using protocol layer
            let routing_key = self.protocol.extract_routing_key(&request);
            info!("{:?}", routing_key);

            // TODO?: could impl middleware here

            // Get handler according to routing (according to protocol layer)
            let Some(handler) = self.router.get_request_handler(&routing_key) else {
                debug!("No handler found for: {:?}", routing_key);
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
        pub fn $method<H>(self, path: &str, handler: H) -> Self
        where
            H: ToHandler<HttpProtocol>,
        {
            self.route((path.into(), RequestMethod::$request_method), handler)
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
}

// TODO: allow handlers that return Result<JSON, Error(?)>
// Really we don't have to do that here, we just need to
// impl Into<HttpResponse> for JSON
// But we DO need to handle funcs that return Result
// actually maybe not, kinda cba

// TODO?: serde, maybe make our own ToJson trait so user
// can use any json lib they want - that we support (with feature flags)
