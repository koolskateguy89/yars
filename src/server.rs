use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use log::{debug, error, info};
use tokio::net::ToSocketAddrs;

use crate::{
    protocol::{HttpProtocol, Protocol, ToHandler},
    router::Router,
    transport::{TcpTransport, TracedConnection, Transport},
    Result,
};

// TODO(finish): some sort of trace/id for each connection for easier log reading

// TODO: some sort of config file: max_connections, max_request_size, etc
// TODO? type safe builder for build YarsServer when have more options

// TODO: doc comment with example usage
/// Logging should be done in the transport.
///
/// Server events are logged using the [log] crate. A log implementation must be provided by the user.
/// For example, [pretty_env_logger].
///
/// As this is an asychronous server, the server will spawn a new task for each connection. Thus
/// the protocol and transport layers and their associated types all need to be [`Send`] and
/// [`Sync`].
#[derive(Debug)]
pub struct YarsServer<T, P>
where
    T: Transport,
    P: Protocol,
{
    transport: T,
    protocol: P,
    router: Router<P>,
    connection_counter: AtomicUsize,
}

impl YarsServer<TcpTransport, HttpProtocol> {
    /// Instantiates a HTTP server that accepts TCP connections
    pub fn default_server() -> Self {
        YarsServer {
            transport: TcpTransport::new(),
            protocol: HttpProtocol,
            router: Router::new(),
            connection_counter: AtomicUsize::new(0),
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
            connection_counter: AtomicUsize::new(0),
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

        self.transport.bind(addr).await?;

        let server = Arc::new(self);
        let mut connection_handles = Vec::new();

        loop {
            let connection_id = server.connection_counter.fetch_add(1, Relaxed);

            // Accept connection with transport layer (in main loop)
            let conn = server.transport.accept(connection_id).await?;

            // Wrap connection with connection id
            let conn = TracedConnection::new(conn, connection_id);

            // Handle connection in new task
            let server = server.clone();
            let handle = tokio::spawn(async move {
                if let Err(e) = server.handle_connection(conn).await {
                    error!("{connection_id}: Error handling connection: {e}",);
                }
            });

            // TODO?: store handles with an id - is there a point?
            connection_handles.push(handle);
        }

        // TODO?: close all tasks
        // self.transport.close(conn).await?;
    }

    async fn handle_connection(&self, mut conn: TracedConnection<T::Connection>) -> Result<()> {
        // Read request from connection with transport layer
        let raw_request = self.transport.read(&mut conn).await?;

        // Parse request bytes using protocol layer
        let Some(request) = self.protocol.parse_request(&raw_request) else {
            debug!("{}: Failed to parse request", conn.id);
            return Ok(());
        };

        // Extract routing key using protocol layer
        let routing_key = self.protocol.extract_routing_key(&request);
        info!("{}: route={routing_key}", conn.id);

        // TODO?: could impl middleware here

        // Get handler according to routing (according to protocol layer)
        let Some(handler) = self.router.get_request_handler(&routing_key) else {
            debug!("{}: No handler found for: {routing_key}", conn.id);
            return Ok(());
        };

        // Handle request by calling handler
        let response = handler(request).map_err(crate::Error::Handler)?;

        // Serialize response using protocol layer
        let response_bytes = self.protocol.serialize_response(&response);

        // Write response bytes to connection with transport layer
        self.transport.write(&mut conn, &response_bytes).await?;

        Ok(())
    }
}

macro_rules! http_method {
    ($method:ident, $request_method:ident) => {
        #[doc = concat!("Registers a `", stringify!($request_method), "` request handler that serves `path` by calling `handler`")]
        pub fn $method(self, path: impl ToString, handler: impl ToHandler<HttpProtocol>) -> Self {
            self.route((path, crate::http::RequestMethod::$request_method), handler)
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
