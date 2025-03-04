#![allow(dead_code)]

use log::{debug, info};
use std::collections::HashMap;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::protocol::{self, HttpProtocol, Protocol};
use crate::request::{HttpRequest, RequestMethod};
use crate::response::HttpResponse;
use crate::router::Router;
use crate::transport::{TcpTransport, Transport};
use crate::Result;

macro_rules! http_method {
    ($method:ident, $request_method:ident) => {
        #[doc = concat!("Registers a `", stringify!($request_method), "` request handler that serves `path` by calling `handler`")]
        pub fn $method<H>(self, path: &str, handler: H) -> Self
        where
            H: protocol::ToHandler<HttpProtocol>,
        {
            self.route((path.into(), RequestMethod::$request_method), handler)
        }
    };
}

// TODO! (fixme) breaks after 1 connection
// todo: some sort of trace/id for each connection for easier log reading

// TODO: allow async
type Handler = dyn Sync + Send + Fn(HttpRequest) -> HttpResponse;

// TODO: make routing not HTTP specific like it currently is
// TODO: directly import handler from protocol once done with generic impl.
// TODO?: some sort of builder for picking transport/protocol
pub struct YarsServer<T, P>
where
    T: Transport,
    P: Protocol,
{
    transport: T,
    protocol: P,
    router: Router<P>,
    handlers: HashMap<(String, RequestMethod), Box<protocol::Handler<P>>>,
    default_handler: Option<Box<protocol::Handler<P>>>,
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
        H: protocol::ToHandler<P>,
    {
        self.router.add_route(routing_key, handler);
        self
    }

    pub fn default_handler<H>(mut self, handler: H) -> Self
    where
        H: protocol::ToHandler<P>,
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

            // Get handler according to routing (according to protocl layer)
            let Some(handler) = self
                .router
                .get_request_handler(self.protocol.extract_routing_key(&request))
            else {
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
}

// TODO: some sort of config file
// TODO: split out this structs
#[derive(Default)]
pub struct HttpServer {
    handlers: HashMap<(String, RequestMethod), Box<Handler>>,
    default_handler: Option<Box<Handler>>,
}

// TODO: some macro(?) like #[get("/")] or #[post("/")]
// fn index() -> HttpResponse {
// will make the function into a struct that impls ToHandler

macro_rules! method {
    ($method:ident, $request_method:ident) => {
        #[doc = concat!("Registers a `", stringify!($request_method), "` request handler that serves `path` by calling `handler`")]
        pub fn $method<T>(self, path: &str, handler: T) -> Self
        where
            T: ToHandler,
        {
            self.route(path, RequestMethod::$request_method, handler)
        }
    };
}

impl HttpServer {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_request_handler(&self, req: &HttpRequest) -> Option<&Handler> {
        let boxed_hander_opt = self.handlers.get(&(req.uri.clone(), req.method));

        // Default handler if no matching handler
        let boxed_hander_opt = boxed_hander_opt.or(self.default_handler.as_ref());

        // Extract a reference to the handler from the Box
        let handler_opt = boxed_hander_opt.map(|boxed_handler| {
            let handler: &Handler = boxed_handler.as_ref();
            handler
        });

        handler_opt
    }

    async fn handle_connection(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buf = Vec::with_capacity(1024);
        stream.read_buf(&mut buf).await?;

        if buf.is_empty() {
            return Ok(());
        }

        debug!(
            "bytes read from connection with {}: {}",
            stream.peer_addr()?,
            buf.len()
        );

        // TODO: not this lol
        let buf_vec = buf.to_vec();
        let buf_str = String::from_utf8(buf_vec).unwrap();

        // TODO: make that take Vec<u8>
        let Some(req) = HttpRequest::parse_request(buf_str) else {
            return Ok(());
        };
        info!("{:?} {}", req.method, req.uri);

        let Some(handler) = self.get_request_handler(&req) else {
            debug!("No handler found for URI: {}", req.uri);
            // TODO?: 404 here
            return Ok(());
        };

        let response = handler(req);

        stream.write_all(&response.status_line()).await?;
        stream.write_all(&response.headers()).await?;
        if let Some(body) = response.body() {
            stream.write_all(body).await?;
        }

        Ok(())
    }

    pub async fn listen<A: ToSocketAddrs>(self, addr: A) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;

        info!("listening on {}", listener.local_addr()?);

        // accept connections and process them serially
        loop {
            let (stream, _) = listener.accept().await?;
            self.handle_connection(stream).await?;
            // FIXME: How can i make this actually async... compiler keeps complaining
            // TODO?: mutex/arc/smthn of self
            // tokio::spawn(async move {
            //     if let Err(_e) = self.handle_connection(stream).await {
            //         println!("wtf");
            //     }
            // });
        }
    }

    /// Adds a route with the given `path` and `method` that will call the given `handler`
    pub fn route<T>(mut self, path: &str, method: RequestMethod, handler: T) -> Self
    where
        T: ToHandler,
    {
        self.handlers
            .insert((path.to_string(), method), handler.to_handler());
        self
    }

    method!(get, GET);
    method!(post, POST);
    method!(put, PUT);
    method!(delete, DELETE);
    method!(head, HEAD);
    method!(options, OPTIONS);
    method!(connect, CONNECT);
    method!(trace, TRACE);
    method!(patch, PATCH);

    pub fn default_handler<T>(mut self, handler: T) -> Self
    where
        T: ToHandler,
    {
        self.default_handler = Some(handler.to_handler());
        self
    }
}

pub trait ToHandler {
    fn to_handler(self) -> Box<Handler>;
}

impl<T, B> ToHandler for T
where
    T: Sync + Send + Fn(HttpRequest) -> B + 'static,
    B: Into<HttpResponse>,
{
    fn to_handler(self) -> Box<Handler> {
        Box::new(move |req| self(req).into())
    }
}

// TODO: funcs that return Result<JSON, Error(?)>
// Really we don't have to do that here, we just need to
// impl Into<HttpResponse> for JSON
// But we DO need to handle funcs that return Result
// actually maybe not, kinda cba

// TODO?: serde, maybe make our own ToJson trait so user
// can use any json lib they want - that we support (with feature flags)
