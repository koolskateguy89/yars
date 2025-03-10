use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use tokio::net::ToSocketAddrs;
use tracing::{debug, error, error_span, info, info_span, trace, Instrument};

use crate::{
    protocol::{HttpProtocol, Protocol, ToHandler},
    router::Router,
    transport::{TcpTransport, Transport},
    Result,
};

// TODO: nom for http req parsing
// TODO: graceful shutdown

// TODO: some sort of config file: max_connections, max_request_size, etc
// TODO? type safe builder for build YarsServer when have more options

// TODO: doc comment with example usage
/// Logging should be done in the transport.
///
/// Server events are logged using the [tracing] crate. A subscriber must be set up by the user.
///
/// As this is an asychronous server, the server will spawn a new task for each connection. Thus
/// the protocol and transport layers and their associated types all need to be [`Send`] and
/// [`Sync`].
///
/// ## Example Usage
///
/// ```rust
/// use yars::YarsServer;
/// use yars::http::{HttpRequest, HttpResponse};
///
/// fn index(_req: HttpRequest) -> anyhow::Result<impl Into<HttpResponse>> {
///     Ok(HttpResponse::Ok().body("Hello, world!"))
/// }
///
/// #[tokio::main]
/// async fn main() -> yars::Result<()> {
///     tracing_subscriber::fmt().init();
///
///     YarsServer::default_server()
///         .get("/", index)
///         .listen("127.0.0.1:8080")
///         .await
/// }
#[derive(Debug)]
pub struct YarsServer<T, P>
where
    T: Transport,
    P: Protocol,
{
    transport: T,
    protocol: P,
    router: Router<P>,
    conn_counter: AtomicUsize,
}

impl YarsServer<TcpTransport, HttpProtocol> {
    /// Instantiates a HTTP server that accepts TCP connections
    pub fn default_server() -> Self {
        YarsServer {
            transport: TcpTransport::new(),
            protocol: HttpProtocol,
            router: Router::new(),
            conn_counter: AtomicUsize::new(0),
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
            conn_counter: AtomicUsize::new(0),
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
            let conn_id = server.conn_counter.fetch_add(1, Relaxed);
            // TODO?: also include remote addr - but then that would have to get it from transport.accept
            // tbh could use empty value and let transport layer handle it
            // actually no cos then we would have to pass the span to the transport layer
            // https://docs.rs/tracing/latest/tracing/#recording-fields
            let conn_span = error_span!("connection", id = conn_id);
            // Enter the span before accepting connection so the connection ID is included in
            // transport layer logs, which could include peer/remote address
            let _entered = conn_span.enter();

            // Accept connection with transport layer (in main loop)
            let conn = server.transport.accept().await?;

            // Handle connection in new task
            let server = server.clone();
            let handle = tokio::spawn(
                async move {
                    if let Err(e) = server.handle_connection(conn).await {
                        error!(?e, "Error handling connection",);
                    }
                }
                .in_current_span(),
            );

            // TODO?: store handles with an id - is there a point?
            connection_handles.push(handle);
        }

        // TODO?: close all tasks
        // self.transport.close(conn).await?;
    }

    async fn handle_connection(&self, mut conn: T::Connection) -> Result<()> {
        // Read request from connection with transport layer
        trace!("Attempting to read from connection");
        let raw_request = self
            .transport
            .read(&mut conn)
            .instrument(info_span!("read_connection"))
            .await?;

        // Parse request bytes using protocol layer
        trace!(len = raw_request.len(), "Parsing request");
        let Some(request) = self.protocol.parse_request(raw_request) else {
            info!("Failed to parse request");
            return Ok(());
        };

        // Extract routing key using protocol layer
        trace!("Extracting routing key");
        let routing_key = self.protocol.extract_routing_key(&request);
        info!(route = %routing_key);

        // TODO?: could impl middleware here

        // Get handler according to routing (according to protocol layer)
        let Some(handler) = self.router.get_request_handler(&routing_key) else {
            info!(route = %routing_key, "No handler found");
            return Ok(());
        };

        // Handle request by calling handler
        let handler_span = info_span!("handle_request");
        let response = handler_span
            .in_scope(|| handler(request))
            .map_err(crate::Error::Handler)?;

        // Serialize response using protocol layer
        let response_bytes = self.protocol.serialize_response(&response);

        // Write response bytes to connection with transport layer
        trace!("Attempting to write to connection");
        self.transport
            .write(&mut conn, &response_bytes)
            .instrument(info_span!("write_connection"))
            .await?;

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
// fn index() -> Result<HttpResponse> {}
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

// Really we don't have to do that here, we just need to
// impl Into<HttpResponse> for JSON

// TODO?: serde, maybe make our own ToJson trait so user
// can use any json lib they want - that we support (with feature flags)
