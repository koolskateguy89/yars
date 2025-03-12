use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

use tokio::{net::ToSocketAddrs, signal, task::JoinHandle};
use tracing::{debug, error, error_span, info, info_span, trace, warn, Instrument};

use crate::{
    protocol::{HttpProtocol, Protocol, ToHandler},
    router::Router,
    transport::{TcpTransport, Transport},
    Result,
};

// TODO: async handlers -> will have to change a lot of examples

// TODO: some sort of config file: max_connections, max_request_size, etc
// TODO? type safe builder for build YarsServer when have more options

/// The main entrypoint for the Yars server.
///
/// Logging should be done in the transport.
///
/// Server events are logged using the [tracing] crate. A subscriber must be set up by the user.
///
/// As this is an asychronous server, the server will spawn a new task for each connection. Thus
/// the protocol and transport layers and their associated types all need to be [`Send`] and
/// [`Sync`].
///
/// # Example Usage
/// ```rust
/// use yars::{
///     http::{HttpRequest, HttpResponse, RequestMethod},
///     protocol::HttpProtocol,
///     transport::TcpTransport,
///     YarsServer,
/// };
///
/// // Handlers can return any Result<Into<HttpResponse>, Into<Box<dyn std::error::Error>>>
/// fn hello(_req: HttpRequest) -> anyhow::Result<HttpResponse> {
///     Ok(HttpResponse::Ok().text("Hello, World!"))
/// }
///
/// fn not_found(_req: HttpRequest) -> yars::Result<HttpResponse> {
///     Ok(HttpResponse::NotFound().text("Not Found"))
/// }
///
/// #[tokio::main]
/// async fn main() -> yars::Result<()> {
///     tracing_subscriber::fmt()
///         .with_target(false)
///         .with_max_level(tracing::Level::INFO)
///         .init();
///
///     YarsServer::new(TcpTransport::default(), HttpProtocol)
///         .route(("/", RequestMethod::GET), hello)
///         // note http protocol exposes shorthand helpers
///         .get("/hello", hello)
///         // can define a default handler for all unhandled routes
///         .default_handler(not_found)
///         .listen("127.0.0.1:8000")
///         .await
/// }
/// ```
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

// Our default is a HTTP server that accepts TCP connections
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
    /// Instantiate a new Yars server with the given transport and protocol
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

    /// Set the default handler, which will be called if no route is found for a request
    pub fn default_handler(mut self, handler: impl ToHandler<P>) -> Self {
        self.router.set_default_handler(handler);
        self
    }

    /// Starts the server. This will bind the transport to the given address and start listening
    /// for incoming connections.
    pub async fn listen<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        // TODO?: debug print type of transport and protocol
        debug!("{:#?}", self.router);

        self.transport.bind(addr).await?;

        // Tbh I have no idea if I am doing this correctly
        // a TaskTracker or similar may be better
        // https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html
        let mut conn_handles = Vec::new();

        tokio::select! {
            _ = self.listen_inner(&mut conn_handles) => {
                // This should never happen
                info!("Server shutting down");
            },
            _ = signal::ctrl_c() => {
                info!("Received SIGINT, shutting down");
            },
        }

        // TODO: close/abort all open connection tasks
        // idk if clearing will actually close the tasks - we don't even need to clear really, can just drop
        // but then really is there a point to storing them in the first place?
        conn_handles.clear();

        // self.transport.close().await?;
        Ok(())
    }

    async fn listen_inner(self, conn_handles: &mut Vec<JoinHandle<()>>) -> Result<()> {
        let server = Arc::new(self);

        loop {
            let conn_id = server.conn_counter.fetch_add(1, Relaxed);
            // TODO?: also include remote addr - but then that would have to get it from transport.accept
            // tbh could use empty value and let transport layer handle it
            // actually no cos then we would have to pass the span to the transport layer
            // https://docs.rs/tracing/latest/tracing/#recording-fields
            // TODO?: route as later param - but how would we pass span to task?
            let conn_span = error_span!("connection", id = conn_id);
            // Enter the span before accepting connection so the connection ID is included in
            // transport layer logs, which could include peer/remote address
            let _entered = conn_span.enter();

            // Accept connection with transport layer
            let mut conn = server.transport.accept().await?;

            // Handle connection in new task
            let server = server.clone();
            let handle = tokio::spawn(
                async move {
                    if let Err(e) = server.handle_connection(&mut conn).await {
                        error!(?e, "Error handling connection");
                    }
                    if let Err(e) = server.transport.shutdown_conn(conn).await {
                        error!(?e, "Error shutting down connection");
                    }
                }
                .in_current_span(),
            );

            conn_handles.push(handle);
        }
    }

    async fn handle_connection(&self, conn: &mut T::Connection) -> Result<()> {
        // Read request from connection with transport layer
        trace!("Attempting to read from connection");
        let raw_request = self
            .transport
            .read(conn)
            .instrument(info_span!("read_connection"))
            .await?;

        if raw_request.is_empty() {
            debug!("Empty request, maybe connection closed");
            return Ok(());
        }

        // Parse request bytes using protocol layer
        trace!(len = raw_request.len(), "Parsing request");
        let Some(request) = self.protocol.parse_request(raw_request) else {
            warn!("Failed to parse request");
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
            .write(conn, &response_bytes)
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

// TODO: some proc(?) macro(?) like #[get("/")] or #[post("/")]
// fn index() -> Result<HttpResponse> {}
// will make the function into a struct that impls ToHandler

/// HTTP specific methods
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
